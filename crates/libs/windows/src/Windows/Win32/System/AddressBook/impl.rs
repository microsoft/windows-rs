pub trait IABContainerImpl: Sized + IMAPIContainerImpl + IMAPIPropImpl {
    fn CreateEntry();
    fn CopyEntries();
    fn DeleteEntries();
    fn ResolveNames();
}
pub trait IAddrBookImpl: Sized + IMAPIPropImpl {
    fn OpenEntry();
    fn CompareEntryIDs();
    fn Advise();
    fn Unadvise();
    fn CreateOneOff();
    fn NewEntry();
    fn ResolveName();
    fn Address();
    fn Details();
    fn RecipOptions();
    fn QueryDefaultRecipOpt();
    fn GetPAB();
    fn SetPAB();
    fn GetDefaultDir();
    fn SetDefaultDir();
    fn GetSearchPath();
    fn SetSearchPath();
    fn PrepareRecips();
}
pub trait IAttachImpl: Sized + IMAPIPropImpl {}
pub trait IDistListImpl: Sized + IMAPIContainerImpl + IMAPIPropImpl {
    fn CreateEntry();
    fn CopyEntries();
    fn DeleteEntries();
    fn ResolveNames();
}
pub trait IMAPIAdviseSinkImpl: Sized {
    fn OnNotify();
}
pub trait IMAPIContainerImpl: Sized + IMAPIPropImpl {
    fn GetContentsTable();
    fn GetHierarchyTable();
    fn OpenEntry();
    fn SetSearchCriteria();
    fn GetSearchCriteria();
}
pub trait IMAPIControlImpl: Sized {
    fn GetLastError();
    fn Activate();
    fn GetState();
}
pub trait IMAPIFolderImpl: Sized + IMAPIContainerImpl + IMAPIPropImpl {
    fn CreateMessage();
    fn CopyMessages();
    fn DeleteMessages();
    fn CreateFolder();
    fn CopyFolder();
    fn DeleteFolder();
    fn SetReadFlags();
    fn GetMessageStatus();
    fn SetMessageStatus();
    fn SaveContentsSort();
    fn EmptyFolder();
}
pub trait IMAPIProgressImpl: Sized {
    fn Progress();
    fn GetFlags();
    fn GetMax();
    fn GetMin();
    fn SetLimits();
}
pub trait IMAPIPropImpl: Sized {
    fn GetLastError();
    fn SaveChanges();
    fn GetProps();
    fn GetPropList();
    fn OpenProperty();
    fn SetProps();
    fn DeleteProps();
    fn CopyTo();
    fn CopyProps();
    fn GetNamesFromIDs();
    fn GetIDsFromNames();
}
pub trait IMAPIStatusImpl: Sized + IMAPIPropImpl {
    fn ValidateState();
    fn SettingsDialog();
    fn ChangePassword();
    fn FlushQueues();
}
pub trait IMAPITableImpl: Sized {
    fn GetLastError();
    fn Advise();
    fn Unadvise();
    fn GetStatus();
    fn SetColumns();
    fn QueryColumns();
    fn GetRowCount();
    fn SeekRow();
    fn SeekRowApprox();
    fn QueryPosition();
    fn FindRow();
    fn Restrict();
    fn CreateBookmark();
    fn FreeBookmark();
    fn SortTable();
    fn QuerySortOrder();
    fn QueryRows();
    fn Abort();
    fn ExpandRow();
    fn CollapseRow();
    fn WaitForCompletion();
    fn GetCollapseState();
    fn SetCollapseState();
}
pub trait IMailUserImpl: Sized + IMAPIPropImpl {}
pub trait IMessageImpl: Sized + IMAPIPropImpl {
    fn GetAttachmentTable();
    fn OpenAttach();
    fn CreateAttach();
    fn DeleteAttach();
    fn GetRecipientTable();
    fn ModifyRecipients();
    fn SubmitMessage();
    fn SetReadFlag();
}
pub trait IMsgStoreImpl: Sized + IMAPIPropImpl {
    fn Advise();
    fn Unadvise();
    fn CompareEntryIDs();
    fn OpenEntry();
    fn SetReceiveFolder();
    fn GetReceiveFolder();
    fn GetReceiveFolderTable();
    fn StoreLogoff();
    fn AbortSubmit();
    fn GetOutgoingQueue();
    fn SetLockState();
    fn FinishedMsg();
    fn NotifyNewMail();
}
pub trait IProfSectImpl: Sized + IMAPIPropImpl {}
pub trait IPropDataImpl: Sized + IMAPIPropImpl {
    fn HrSetObjAccess();
    fn HrSetPropAccess();
    fn HrGetPropAccess();
    fn HrAddObjProps();
}
pub trait IProviderAdminImpl: Sized {
    fn GetLastError();
    fn GetProviderTable();
    fn CreateProvider();
    fn DeleteProvider();
    fn OpenProfileSection();
}
pub trait ITableDataImpl: Sized {
    fn HrGetView();
    fn HrModifyRow();
    fn HrDeleteRow();
    fn HrQueryRow();
    fn HrEnumRow();
    fn HrNotify();
    fn HrInsertRow();
    fn HrModifyRows();
    fn HrDeleteRows();
}
pub trait IWABExtInitImpl: Sized {
    fn Initialize();
}
pub trait IWABOBJECT_Impl: Sized {
    fn QueryInterface();
    fn AddRef();
    fn Release();
    fn GetLastError();
    fn AllocateBuffer();
    fn AllocateMore();
    fn FreeBuffer();
    fn Backup();
    fn Import();
    fn Find();
    fn VCardDisplay();
    fn LDAPUrl();
    fn VCardCreate();
    fn VCardRetrieve();
    fn GetMe();
    fn SetMe();
}
pub trait IWABObjectImpl: Sized {
    fn GetLastError();
    fn AllocateBuffer();
    fn AllocateMore();
    fn FreeBuffer();
    fn Backup();
    fn Import();
    fn Find();
    fn VCardDisplay();
    fn LDAPUrl();
    fn VCardCreate();
    fn VCardRetrieve();
    fn GetMe();
    fn SetMe();
}
