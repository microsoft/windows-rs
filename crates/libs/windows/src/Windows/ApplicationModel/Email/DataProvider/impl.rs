#[cfg(feature = "implement_exclusive")]
pub trait IEmailDataProviderConnectionImpl: Sized {
    fn MailboxSyncRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxSyncManagerSyncRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMailboxSyncRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadMessageRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDownloadMessageRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadMessageRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadAttachmentRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDownloadAttachmentRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadAttachmentRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateFolderRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxCreateFolderRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveCreateFolderRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DeleteFolderRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDeleteFolderRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDeleteFolderRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EmptyFolderRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxEmptyFolderRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEmptyFolderRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MoveFolderRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxMoveFolderRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveMoveFolderRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn UpdateMeetingResponseRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxUpdateMeetingResponseRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdateMeetingResponseRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ForwardMeetingRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxForwardMeetingRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveForwardMeetingRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ProposeNewTimeForMeetingRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxProposeNewTimeForMeetingRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveProposeNewTimeForMeetingRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetAutoReplySettingsRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxSetAutoReplySettingsRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSetAutoReplySettingsRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetAutoReplySettingsRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxGetAutoReplySettingsRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGetAutoReplySettingsRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ResolveRecipientsRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxResolveRecipientsRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveResolveRecipientsRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ValidateCertificatesRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxValidateCertificatesRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveValidateCertificatesRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ServerSearchReadBatchRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxServerSearchReadBatchRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveServerSearchReadBatchRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailDataProviderConnection {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailDataProviderConnection";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailDataProviderConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>() -> IEmailDataProviderConnectionVtbl {
        unsafe extern "system" fn MailboxSyncRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MailboxSyncRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxSyncManagerSyncRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxSyncManagerSyncRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMailboxSyncRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMailboxSyncRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DownloadMessageRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadMessageRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDownloadMessageRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDownloadMessageRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDownloadMessageRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDownloadMessageRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DownloadAttachmentRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DownloadAttachmentRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDownloadAttachmentRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDownloadAttachmentRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDownloadAttachmentRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDownloadAttachmentRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn CreateFolderRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFolderRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxCreateFolderRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxCreateFolderRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveCreateFolderRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveCreateFolderRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn DeleteFolderRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteFolderRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDeleteFolderRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxDeleteFolderRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDeleteFolderRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDeleteFolderRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn EmptyFolderRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmptyFolderRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxEmptyFolderRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxEmptyFolderRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEmptyFolderRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEmptyFolderRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn MoveFolderRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveFolderRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxMoveFolderRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxMoveFolderRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMoveFolderRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMoveFolderRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn UpdateMeetingResponseRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateMeetingResponseRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxUpdateMeetingResponseRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxUpdateMeetingResponseRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveUpdateMeetingResponseRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveUpdateMeetingResponseRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ForwardMeetingRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForwardMeetingRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxForwardMeetingRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxForwardMeetingRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveForwardMeetingRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveForwardMeetingRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ProposeNewTimeForMeetingRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProposeNewTimeForMeetingRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxProposeNewTimeForMeetingRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxProposeNewTimeForMeetingRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProposeNewTimeForMeetingRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProposeNewTimeForMeetingRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SetAutoReplySettingsRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAutoReplySettingsRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxSetAutoReplySettingsRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxSetAutoReplySettingsRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveSetAutoReplySettingsRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSetAutoReplySettingsRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetAutoReplySettingsRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutoReplySettingsRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxGetAutoReplySettingsRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxGetAutoReplySettingsRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveGetAutoReplySettingsRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveGetAutoReplySettingsRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResolveRecipientsRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolveRecipientsRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxResolveRecipientsRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxResolveRecipientsRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveResolveRecipientsRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveResolveRecipientsRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ValidateCertificatesRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateCertificatesRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxValidateCertificatesRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxValidateCertificatesRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveValidateCertificatesRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveValidateCertificatesRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ServerSearchReadBatchRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServerSearchReadBatchRequested(&*(&handler as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxServerSearchReadBatchRequestEventArgs> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TypedEventHandler<EmailDataProviderConnection, EmailMailboxServerSearchReadBatchRequestEventArgs> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveServerSearchReadBatchRequested<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveServerSearchReadBatchRequested(&*(&token as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::EventRegistrationToken as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Start<Impl: IEmailDataProviderConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailDataProviderConnection>,
            ::windows::core::GetTrustLevel,
            MailboxSyncRequested::<Impl, OFFSET>,
            RemoveMailboxSyncRequested::<Impl, OFFSET>,
            DownloadMessageRequested::<Impl, OFFSET>,
            RemoveDownloadMessageRequested::<Impl, OFFSET>,
            DownloadAttachmentRequested::<Impl, OFFSET>,
            RemoveDownloadAttachmentRequested::<Impl, OFFSET>,
            CreateFolderRequested::<Impl, OFFSET>,
            RemoveCreateFolderRequested::<Impl, OFFSET>,
            DeleteFolderRequested::<Impl, OFFSET>,
            RemoveDeleteFolderRequested::<Impl, OFFSET>,
            EmptyFolderRequested::<Impl, OFFSET>,
            RemoveEmptyFolderRequested::<Impl, OFFSET>,
            MoveFolderRequested::<Impl, OFFSET>,
            RemoveMoveFolderRequested::<Impl, OFFSET>,
            UpdateMeetingResponseRequested::<Impl, OFFSET>,
            RemoveUpdateMeetingResponseRequested::<Impl, OFFSET>,
            ForwardMeetingRequested::<Impl, OFFSET>,
            RemoveForwardMeetingRequested::<Impl, OFFSET>,
            ProposeNewTimeForMeetingRequested::<Impl, OFFSET>,
            RemoveProposeNewTimeForMeetingRequested::<Impl, OFFSET>,
            SetAutoReplySettingsRequested::<Impl, OFFSET>,
            RemoveSetAutoReplySettingsRequested::<Impl, OFFSET>,
            GetAutoReplySettingsRequested::<Impl, OFFSET>,
            RemoveGetAutoReplySettingsRequested::<Impl, OFFSET>,
            ResolveRecipientsRequested::<Impl, OFFSET>,
            RemoveResolveRecipientsRequested::<Impl, OFFSET>,
            ValidateCertificatesRequested::<Impl, OFFSET>,
            RemoveValidateCertificatesRequested::<Impl, OFFSET>,
            ServerSearchReadBatchRequested::<Impl, OFFSET>,
            RemoveServerSearchReadBatchRequested::<Impl, OFFSET>,
            Start::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailDataProviderTriggerDetailsImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<EmailDataProviderConnection>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailDataProviderTriggerDetails {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailDataProviderTriggerDetails";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailDataProviderTriggerDetailsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailDataProviderTriggerDetailsImpl, const OFFSET: isize>() -> IEmailDataProviderTriggerDetailsVtbl {
        unsafe extern "system" fn Connection<Impl: IEmailDataProviderTriggerDetailsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailDataProviderTriggerDetails>, ::windows::core::GetTrustLevel, Connection::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxCreateFolderRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ParentFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self, folder: &::core::option::Option<super::EmailFolder>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self, status: super::EmailMailboxCreateFolderStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxCreateFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxCreateFolderRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxCreateFolderRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxCreateFolderRequestImpl, const OFFSET: isize>() -> IEmailMailboxCreateFolderRequestVtbl {
        unsafe extern "system" fn EmailMailboxId<Impl: IEmailMailboxCreateFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentFolderId<Impl: IEmailMailboxCreateFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ParentFolderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IEmailMailboxCreateFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IEmailMailboxCreateFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, folder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync(&*(&folder as *const <super::EmailFolder as ::windows::core::Abi>::Abi as *const <super::EmailFolder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IEmailMailboxCreateFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: super::EmailMailboxCreateFolderStatus, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync(status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxCreateFolderRequest>, ::windows::core::GetTrustLevel, EmailMailboxId::<Impl, OFFSET>, ParentFolderId::<Impl, OFFSET>, Name::<Impl, OFFSET>, ReportCompletedAsync::<Impl, OFFSET>, ReportFailedAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxCreateFolderRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxCreateFolderRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxCreateFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxCreateFolderRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxCreateFolderRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxCreateFolderRequestEventArgsImpl, const OFFSET: isize>() -> IEmailMailboxCreateFolderRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IEmailMailboxCreateFolderRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IEmailMailboxCreateFolderRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxCreateFolderRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxDeleteFolderRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self, status: super::EmailMailboxDeleteFolderStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxDeleteFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDeleteFolderRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxDeleteFolderRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxDeleteFolderRequestImpl, const OFFSET: isize>() -> IEmailMailboxDeleteFolderRequestVtbl {
        unsafe extern "system" fn EmailMailboxId<Impl: IEmailMailboxDeleteFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmailFolderId<Impl: IEmailMailboxDeleteFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailFolderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IEmailMailboxDeleteFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IEmailMailboxDeleteFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: super::EmailMailboxDeleteFolderStatus, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync(status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxDeleteFolderRequest>, ::windows::core::GetTrustLevel, EmailMailboxId::<Impl, OFFSET>, EmailFolderId::<Impl, OFFSET>, ReportCompletedAsync::<Impl, OFFSET>, ReportFailedAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxDeleteFolderRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxDeleteFolderRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxDeleteFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDeleteFolderRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxDeleteFolderRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxDeleteFolderRequestEventArgsImpl, const OFFSET: isize>() -> IEmailMailboxDeleteFolderRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IEmailMailboxDeleteFolderRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IEmailMailboxDeleteFolderRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxDeleteFolderRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxDownloadAttachmentRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailAttachmentId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxDownloadAttachmentRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadAttachmentRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxDownloadAttachmentRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxDownloadAttachmentRequestImpl, const OFFSET: isize>() -> IEmailMailboxDownloadAttachmentRequestVtbl {
        unsafe extern "system" fn EmailMailboxId<Impl: IEmailMailboxDownloadAttachmentRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmailMessageId<Impl: IEmailMailboxDownloadAttachmentRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMessageId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmailAttachmentId<Impl: IEmailMailboxDownloadAttachmentRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailAttachmentId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IEmailMailboxDownloadAttachmentRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IEmailMailboxDownloadAttachmentRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxDownloadAttachmentRequest>, ::windows::core::GetTrustLevel, EmailMailboxId::<Impl, OFFSET>, EmailMessageId::<Impl, OFFSET>, EmailAttachmentId::<Impl, OFFSET>, ReportCompletedAsync::<Impl, OFFSET>, ReportFailedAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxDownloadAttachmentRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxDownloadAttachmentRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxDownloadAttachmentRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadAttachmentRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxDownloadAttachmentRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxDownloadAttachmentRequestEventArgsImpl, const OFFSET: isize>() -> IEmailMailboxDownloadAttachmentRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IEmailMailboxDownloadAttachmentRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IEmailMailboxDownloadAttachmentRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxDownloadAttachmentRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxDownloadMessageRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxDownloadMessageRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadMessageRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxDownloadMessageRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxDownloadMessageRequestImpl, const OFFSET: isize>() -> IEmailMailboxDownloadMessageRequestVtbl {
        unsafe extern "system" fn EmailMailboxId<Impl: IEmailMailboxDownloadMessageRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmailMessageId<Impl: IEmailMailboxDownloadMessageRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMessageId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IEmailMailboxDownloadMessageRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IEmailMailboxDownloadMessageRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxDownloadMessageRequest>, ::windows::core::GetTrustLevel, EmailMailboxId::<Impl, OFFSET>, EmailMessageId::<Impl, OFFSET>, ReportCompletedAsync::<Impl, OFFSET>, ReportFailedAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxDownloadMessageRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxDownloadMessageRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxDownloadMessageRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxDownloadMessageRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxDownloadMessageRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxDownloadMessageRequestEventArgsImpl, const OFFSET: isize>() -> IEmailMailboxDownloadMessageRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IEmailMailboxDownloadMessageRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IEmailMailboxDownloadMessageRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxDownloadMessageRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxEmptyFolderRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self, status: super::EmailMailboxEmptyFolderStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxEmptyFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxEmptyFolderRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxEmptyFolderRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxEmptyFolderRequestImpl, const OFFSET: isize>() -> IEmailMailboxEmptyFolderRequestVtbl {
        unsafe extern "system" fn EmailMailboxId<Impl: IEmailMailboxEmptyFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmailFolderId<Impl: IEmailMailboxEmptyFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailFolderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IEmailMailboxEmptyFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IEmailMailboxEmptyFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: super::EmailMailboxEmptyFolderStatus, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync(status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxEmptyFolderRequest>, ::windows::core::GetTrustLevel, EmailMailboxId::<Impl, OFFSET>, EmailFolderId::<Impl, OFFSET>, ReportCompletedAsync::<Impl, OFFSET>, ReportFailedAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxEmptyFolderRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxEmptyFolderRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxEmptyFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxEmptyFolderRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxEmptyFolderRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxEmptyFolderRequestEventArgsImpl, const OFFSET: isize>() -> IEmailMailboxEmptyFolderRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IEmailMailboxEmptyFolderRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IEmailMailboxEmptyFolderRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxEmptyFolderRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxForwardMeetingRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Recipients(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::EmailRecipient>>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ForwardHeaderType(&self) -> ::windows::core::Result<super::EmailMessageBodyKind>;
    fn ForwardHeader(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxForwardMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxForwardMeetingRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxForwardMeetingRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxForwardMeetingRequestImpl, const OFFSET: isize>() -> IEmailMailboxForwardMeetingRequestVtbl {
        unsafe extern "system" fn EmailMailboxId<Impl: IEmailMailboxForwardMeetingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmailMessageId<Impl: IEmailMailboxForwardMeetingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMessageId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recipients<Impl: IEmailMailboxForwardMeetingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recipients() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subject<Impl: IEmailMailboxForwardMeetingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForwardHeaderType<Impl: IEmailMailboxForwardMeetingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::EmailMessageBodyKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForwardHeaderType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForwardHeader<Impl: IEmailMailboxForwardMeetingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForwardHeader() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comment<Impl: IEmailMailboxForwardMeetingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Comment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IEmailMailboxForwardMeetingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IEmailMailboxForwardMeetingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailMailboxForwardMeetingRequest>,
            ::windows::core::GetTrustLevel,
            EmailMailboxId::<Impl, OFFSET>,
            EmailMessageId::<Impl, OFFSET>,
            Recipients::<Impl, OFFSET>,
            Subject::<Impl, OFFSET>,
            ForwardHeaderType::<Impl, OFFSET>,
            ForwardHeader::<Impl, OFFSET>,
            Comment::<Impl, OFFSET>,
            ReportCompletedAsync::<Impl, OFFSET>,
            ReportFailedAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxForwardMeetingRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxForwardMeetingRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxForwardMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxForwardMeetingRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxForwardMeetingRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxForwardMeetingRequestEventArgsImpl, const OFFSET: isize>() -> IEmailMailboxForwardMeetingRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IEmailMailboxForwardMeetingRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IEmailMailboxForwardMeetingRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxForwardMeetingRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxGetAutoReplySettingsRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RequestedFormat(&self) -> ::windows::core::Result<super::EmailMailboxAutoReplyMessageResponseKind>;
    fn ReportCompletedAsync(&self, autoreplysettings: &::core::option::Option<super::EmailMailboxAutoReplySettings>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxGetAutoReplySettingsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxGetAutoReplySettingsRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxGetAutoReplySettingsRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxGetAutoReplySettingsRequestImpl, const OFFSET: isize>() -> IEmailMailboxGetAutoReplySettingsRequestVtbl {
        unsafe extern "system" fn EmailMailboxId<Impl: IEmailMailboxGetAutoReplySettingsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedFormat<Impl: IEmailMailboxGetAutoReplySettingsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::EmailMailboxAutoReplyMessageResponseKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IEmailMailboxGetAutoReplySettingsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, autoreplysettings: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync(&*(&autoreplysettings as *const <super::EmailMailboxAutoReplySettings as ::windows::core::Abi>::Abi as *const <super::EmailMailboxAutoReplySettings as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IEmailMailboxGetAutoReplySettingsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxGetAutoReplySettingsRequest>, ::windows::core::GetTrustLevel, EmailMailboxId::<Impl, OFFSET>, RequestedFormat::<Impl, OFFSET>, ReportCompletedAsync::<Impl, OFFSET>, ReportFailedAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxGetAutoReplySettingsRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxGetAutoReplySettingsRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxGetAutoReplySettingsRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxGetAutoReplySettingsRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxGetAutoReplySettingsRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxGetAutoReplySettingsRequestEventArgsImpl, const OFFSET: isize>() -> IEmailMailboxGetAutoReplySettingsRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IEmailMailboxGetAutoReplySettingsRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IEmailMailboxGetAutoReplySettingsRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxGetAutoReplySettingsRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxMoveFolderRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NewParentFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NewFolderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxMoveFolderRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxMoveFolderRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxMoveFolderRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxMoveFolderRequestImpl, const OFFSET: isize>() -> IEmailMailboxMoveFolderRequestVtbl {
        unsafe extern "system" fn EmailMailboxId<Impl: IEmailMailboxMoveFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmailFolderId<Impl: IEmailMailboxMoveFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailFolderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewParentFolderId<Impl: IEmailMailboxMoveFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewParentFolderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewFolderName<Impl: IEmailMailboxMoveFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewFolderName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IEmailMailboxMoveFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IEmailMailboxMoveFolderRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxMoveFolderRequest>, ::windows::core::GetTrustLevel, EmailMailboxId::<Impl, OFFSET>, EmailFolderId::<Impl, OFFSET>, NewParentFolderId::<Impl, OFFSET>, NewFolderName::<Impl, OFFSET>, ReportCompletedAsync::<Impl, OFFSET>, ReportFailedAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxMoveFolderRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxMoveFolderRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxMoveFolderRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxMoveFolderRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxMoveFolderRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxMoveFolderRequestEventArgsImpl, const OFFSET: isize>() -> IEmailMailboxMoveFolderRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IEmailMailboxMoveFolderRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IEmailMailboxMoveFolderRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxMoveFolderRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxProposeNewTimeForMeetingRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn NewStartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn NewDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxProposeNewTimeForMeetingRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxProposeNewTimeForMeetingRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxProposeNewTimeForMeetingRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxProposeNewTimeForMeetingRequestImpl, const OFFSET: isize>() -> IEmailMailboxProposeNewTimeForMeetingRequestVtbl {
        unsafe extern "system" fn EmailMailboxId<Impl: IEmailMailboxProposeNewTimeForMeetingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmailMessageId<Impl: IEmailMailboxProposeNewTimeForMeetingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMessageId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewStartTime<Impl: IEmailMailboxProposeNewTimeForMeetingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewDuration<Impl: IEmailMailboxProposeNewTimeForMeetingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subject<Impl: IEmailMailboxProposeNewTimeForMeetingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comment<Impl: IEmailMailboxProposeNewTimeForMeetingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Comment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IEmailMailboxProposeNewTimeForMeetingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IEmailMailboxProposeNewTimeForMeetingRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailMailboxProposeNewTimeForMeetingRequest>,
            ::windows::core::GetTrustLevel,
            EmailMailboxId::<Impl, OFFSET>,
            EmailMessageId::<Impl, OFFSET>,
            NewStartTime::<Impl, OFFSET>,
            NewDuration::<Impl, OFFSET>,
            Subject::<Impl, OFFSET>,
            Comment::<Impl, OFFSET>,
            ReportCompletedAsync::<Impl, OFFSET>,
            ReportFailedAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxProposeNewTimeForMeetingRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxProposeNewTimeForMeetingRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxProposeNewTimeForMeetingRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxProposeNewTimeForMeetingRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxProposeNewTimeForMeetingRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxProposeNewTimeForMeetingRequestEventArgsImpl, const OFFSET: isize>() -> IEmailMailboxProposeNewTimeForMeetingRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IEmailMailboxProposeNewTimeForMeetingRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IEmailMailboxProposeNewTimeForMeetingRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxProposeNewTimeForMeetingRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxResolveRecipientsRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Recipients(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn ReportCompletedAsync(&self, resolutionresults: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::EmailRecipientResolutionResult>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxResolveRecipientsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxResolveRecipientsRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxResolveRecipientsRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxResolveRecipientsRequestImpl, const OFFSET: isize>() -> IEmailMailboxResolveRecipientsRequestVtbl {
        unsafe extern "system" fn EmailMailboxId<Impl: IEmailMailboxResolveRecipientsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recipients<Impl: IEmailMailboxResolveRecipientsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recipients() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IEmailMailboxResolveRecipientsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resolutionresults: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync(&*(&resolutionresults as *const <super::super::super::Foundation::Collections::IIterable<super::EmailRecipientResolutionResult> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<super::EmailRecipientResolutionResult> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IEmailMailboxResolveRecipientsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxResolveRecipientsRequest>, ::windows::core::GetTrustLevel, EmailMailboxId::<Impl, OFFSET>, Recipients::<Impl, OFFSET>, ReportCompletedAsync::<Impl, OFFSET>, ReportFailedAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxResolveRecipientsRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxResolveRecipientsRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxResolveRecipientsRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxResolveRecipientsRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxResolveRecipientsRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxResolveRecipientsRequestEventArgsImpl, const OFFSET: isize>() -> IEmailMailboxResolveRecipientsRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IEmailMailboxResolveRecipientsRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IEmailMailboxResolveRecipientsRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxResolveRecipientsRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxServerSearchReadBatchRequestImpl: Sized {
    fn SessionId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailFolderId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Options(&self) -> ::windows::core::Result<super::EmailQueryOptions>;
    fn SuggestedBatchSize(&self) -> ::windows::core::Result<u32>;
    fn SaveMessageAsync(&self, message: &::core::option::Option<super::EmailMessage>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self, batchstatus: super::EmailBatchStatus) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxServerSearchReadBatchRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxServerSearchReadBatchRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxServerSearchReadBatchRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxServerSearchReadBatchRequestImpl, const OFFSET: isize>() -> IEmailMailboxServerSearchReadBatchRequestVtbl {
        unsafe extern "system" fn SessionId<Impl: IEmailMailboxServerSearchReadBatchRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmailMailboxId<Impl: IEmailMailboxServerSearchReadBatchRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmailFolderId<Impl: IEmailMailboxServerSearchReadBatchRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailFolderId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Options<Impl: IEmailMailboxServerSearchReadBatchRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Options() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuggestedBatchSize<Impl: IEmailMailboxServerSearchReadBatchRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SuggestedBatchSize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveMessageAsync<Impl: IEmailMailboxServerSearchReadBatchRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveMessageAsync(&*(&message as *const <super::EmailMessage as ::windows::core::Abi>::Abi as *const <super::EmailMessage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IEmailMailboxServerSearchReadBatchRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IEmailMailboxServerSearchReadBatchRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, batchstatus: super::EmailBatchStatus, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync(batchstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailMailboxServerSearchReadBatchRequest>,
            ::windows::core::GetTrustLevel,
            SessionId::<Impl, OFFSET>,
            EmailMailboxId::<Impl, OFFSET>,
            EmailFolderId::<Impl, OFFSET>,
            Options::<Impl, OFFSET>,
            SuggestedBatchSize::<Impl, OFFSET>,
            SaveMessageAsync::<Impl, OFFSET>,
            ReportCompletedAsync::<Impl, OFFSET>,
            ReportFailedAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxServerSearchReadBatchRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxServerSearchReadBatchRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxServerSearchReadBatchRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxServerSearchReadBatchRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxServerSearchReadBatchRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxServerSearchReadBatchRequestEventArgsImpl, const OFFSET: isize>() -> IEmailMailboxServerSearchReadBatchRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IEmailMailboxServerSearchReadBatchRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IEmailMailboxServerSearchReadBatchRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxServerSearchReadBatchRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxSetAutoReplySettingsRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AutoReplySettings(&self) -> ::windows::core::Result<super::EmailMailboxAutoReplySettings>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxSetAutoReplySettingsRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSetAutoReplySettingsRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxSetAutoReplySettingsRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxSetAutoReplySettingsRequestImpl, const OFFSET: isize>() -> IEmailMailboxSetAutoReplySettingsRequestVtbl {
        unsafe extern "system" fn EmailMailboxId<Impl: IEmailMailboxSetAutoReplySettingsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AutoReplySettings<Impl: IEmailMailboxSetAutoReplySettingsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AutoReplySettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IEmailMailboxSetAutoReplySettingsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IEmailMailboxSetAutoReplySettingsRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxSetAutoReplySettingsRequest>, ::windows::core::GetTrustLevel, EmailMailboxId::<Impl, OFFSET>, AutoReplySettings::<Impl, OFFSET>, ReportCompletedAsync::<Impl, OFFSET>, ReportFailedAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxSetAutoReplySettingsRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxSetAutoReplySettingsRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxSetAutoReplySettingsRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSetAutoReplySettingsRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxSetAutoReplySettingsRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxSetAutoReplySettingsRequestEventArgsImpl, const OFFSET: isize>() -> IEmailMailboxSetAutoReplySettingsRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IEmailMailboxSetAutoReplySettingsRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IEmailMailboxSetAutoReplySettingsRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxSetAutoReplySettingsRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxSyncManagerSyncRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxSyncManagerSyncRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSyncManagerSyncRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxSyncManagerSyncRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxSyncManagerSyncRequestImpl, const OFFSET: isize>() -> IEmailMailboxSyncManagerSyncRequestVtbl {
        unsafe extern "system" fn EmailMailboxId<Impl: IEmailMailboxSyncManagerSyncRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IEmailMailboxSyncManagerSyncRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IEmailMailboxSyncManagerSyncRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxSyncManagerSyncRequest>, ::windows::core::GetTrustLevel, EmailMailboxId::<Impl, OFFSET>, ReportCompletedAsync::<Impl, OFFSET>, ReportFailedAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxSyncManagerSyncRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxSyncManagerSyncRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxSyncManagerSyncRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxSyncManagerSyncRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxSyncManagerSyncRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxSyncManagerSyncRequestEventArgsImpl, const OFFSET: isize>() -> IEmailMailboxSyncManagerSyncRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IEmailMailboxSyncManagerSyncRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IEmailMailboxSyncManagerSyncRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxSyncManagerSyncRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxUpdateMeetingResponseRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EmailMessageId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Response(&self) -> ::windows::core::Result<super::EmailMeetingResponseType>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Comment(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SendUpdate(&self) -> ::windows::core::Result<bool>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxUpdateMeetingResponseRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxUpdateMeetingResponseRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxUpdateMeetingResponseRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxUpdateMeetingResponseRequestImpl, const OFFSET: isize>() -> IEmailMailboxUpdateMeetingResponseRequestVtbl {
        unsafe extern "system" fn EmailMailboxId<Impl: IEmailMailboxUpdateMeetingResponseRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EmailMessageId<Impl: IEmailMailboxUpdateMeetingResponseRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMessageId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Response<Impl: IEmailMailboxUpdateMeetingResponseRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::EmailMeetingResponseType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Response() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subject<Impl: IEmailMailboxUpdateMeetingResponseRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Subject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Comment<Impl: IEmailMailboxUpdateMeetingResponseRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Comment() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendUpdate<Impl: IEmailMailboxUpdateMeetingResponseRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendUpdate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IEmailMailboxUpdateMeetingResponseRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IEmailMailboxUpdateMeetingResponseRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IEmailMailboxUpdateMeetingResponseRequest>,
            ::windows::core::GetTrustLevel,
            EmailMailboxId::<Impl, OFFSET>,
            EmailMessageId::<Impl, OFFSET>,
            Response::<Impl, OFFSET>,
            Subject::<Impl, OFFSET>,
            Comment::<Impl, OFFSET>,
            SendUpdate::<Impl, OFFSET>,
            ReportCompletedAsync::<Impl, OFFSET>,
            ReportFailedAsync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxUpdateMeetingResponseRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxUpdateMeetingResponseRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxUpdateMeetingResponseRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxUpdateMeetingResponseRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxUpdateMeetingResponseRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxUpdateMeetingResponseRequestEventArgsImpl, const OFFSET: isize>() -> IEmailMailboxUpdateMeetingResponseRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IEmailMailboxUpdateMeetingResponseRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IEmailMailboxUpdateMeetingResponseRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxUpdateMeetingResponseRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxValidateCertificatesRequestImpl: Sized {
    fn EmailMailboxId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Certificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::super::super::Security::Cryptography::Certificates::Certificate>>;
    fn ReportCompletedAsync(&self, validationstatuses: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<super::EmailCertificateValidationStatus>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxValidateCertificatesRequest {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxValidateCertificatesRequest";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxValidateCertificatesRequestVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxValidateCertificatesRequestImpl, const OFFSET: isize>() -> IEmailMailboxValidateCertificatesRequestVtbl {
        unsafe extern "system" fn EmailMailboxId<Impl: IEmailMailboxValidateCertificatesRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EmailMailboxId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Certificates<Impl: IEmailMailboxValidateCertificatesRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Certificates() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportCompletedAsync<Impl: IEmailMailboxValidateCertificatesRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, validationstatuses: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportCompletedAsync(&*(&validationstatuses as *const <super::super::super::Foundation::Collections::IIterable<super::EmailCertificateValidationStatus> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IIterable<super::EmailCertificateValidationStatus> as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReportFailedAsync<Impl: IEmailMailboxValidateCertificatesRequestImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportFailedAsync() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxValidateCertificatesRequest>, ::windows::core::GetTrustLevel, EmailMailboxId::<Impl, OFFSET>, Certificates::<Impl, OFFSET>, ReportCompletedAsync::<Impl, OFFSET>, ReportFailedAsync::<Impl, OFFSET>)
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IEmailMailboxValidateCertificatesRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<EmailMailboxValidateCertificatesRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IEmailMailboxValidateCertificatesRequestEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Email.DataProvider.IEmailMailboxValidateCertificatesRequestEventArgs";
}
#[cfg(feature = "implement_exclusive")]
impl IEmailMailboxValidateCertificatesRequestEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEmailMailboxValidateCertificatesRequestEventArgsImpl, const OFFSET: isize>() -> IEmailMailboxValidateCertificatesRequestEventArgsVtbl {
        unsafe extern "system" fn Request<Impl: IEmailMailboxValidateCertificatesRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Request() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeferral<Impl: IEmailMailboxValidateCertificatesRequestEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeferral() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEmailMailboxValidateCertificatesRequestEventArgs>, ::windows::core::GetTrustLevel, Request::<Impl, OFFSET>, GetDeferral::<Impl, OFFSET>)
    }
}
