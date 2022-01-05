pub trait IDirectWriterLockImpl: Sized {
    fn WaitForWriteAccess();
    fn ReleaseWriteAccess();
    fn HaveWriteAccess();
}
pub trait IEnumSTATPROPSETSTGImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumSTATPROPSTGImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumSTATSTGImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IFillLockBytesImpl: Sized {
    fn FillAppend();
    fn FillAt();
    fn SetFillSize();
    fn Terminate();
}
pub trait ILayoutStorageImpl: Sized {
    fn LayoutScript();
    fn BeginMonitor();
    fn EndMonitor();
    fn ReLayoutDocfile();
    fn ReLayoutDocfileOnILockBytes();
}
pub trait ILockBytesImpl: Sized {
    fn ReadAt();
    fn WriteAt();
    fn Flush();
    fn SetSize();
    fn LockRegion();
    fn UnlockRegion();
    fn Stat();
}
pub trait IPersistStorageImpl: Sized + IPersistImpl {
    fn IsDirty();
    fn InitNew();
    fn Load();
    fn Save();
    fn SaveCompleted();
    fn HandsOffStorage();
}
pub trait IPropertyBagImpl: Sized {
    fn Read();
    fn Write();
}
pub trait IPropertyBag2Impl: Sized {
    fn Read();
    fn Write();
    fn CountProperties();
    fn GetPropertyInfo();
    fn LoadObject();
}
pub trait IPropertySetStorageImpl: Sized {
    fn Create();
    fn Open();
    fn Delete();
    fn Enum();
}
pub trait IPropertyStorageImpl: Sized {
    fn ReadMultiple();
    fn WriteMultiple();
    fn DeleteMultiple();
    fn ReadPropertyNames();
    fn WritePropertyNames();
    fn DeletePropertyNames();
    fn Commit();
    fn Revert();
    fn Enum();
    fn SetTimes();
    fn SetClass();
    fn Stat();
}
pub trait IRootStorageImpl: Sized {
    fn SwitchToFile();
}
pub trait IStorageImpl: Sized {
    fn CreateStream();
    fn OpenStream();
    fn CreateStorage();
    fn OpenStorage();
    fn CopyTo();
    fn MoveElementTo();
    fn Commit();
    fn Revert();
    fn EnumElements();
    fn DestroyElement();
    fn RenameElement();
    fn SetElementTimes();
    fn SetClass();
    fn SetStateBits();
    fn Stat();
}
