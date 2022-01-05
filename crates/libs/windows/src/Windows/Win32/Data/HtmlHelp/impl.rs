pub trait IITDatabaseImpl: Sized {
    fn Open();
    fn Close();
    fn CreateObject();
    fn GetObject();
    fn GetObjectPersistence();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IITPropListImpl: Sized + IPersistStreamInitImpl + IPersistImpl {
    fn Set();
    fn Set();
    fn Set();
    fn Add();
    fn Get();
    fn Clear();
    fn SetPersist();
    fn SetPersist();
    fn GetFirst();
    fn GetNext();
    fn GetPropCount();
    fn SaveHeader();
    fn SaveData();
    fn GetHeaderSize();
    fn GetDataSize();
    fn SaveDataToStream();
    fn LoadFromMem();
    fn SaveToMem();
}
pub trait IITResultSetImpl: Sized {
    fn SetColumnPriority();
    fn SetColumnHeap();
    fn SetKeyProp();
    fn Add();
    fn Add();
    fn Add();
    fn Add();
    fn Append();
    fn Set();
    fn Set();
    fn Set();
    fn Set();
    fn Copy();
    fn AppendRows();
    fn Get();
    fn GetKeyProp();
    fn GetColumnPriority();
    fn GetRowCount();
    fn GetColumnCount();
    fn GetColumn();
    fn GetColumn();
    fn GetColumnFromPropID();
    fn Clear();
    fn ClearRows();
    fn Free();
    fn IsCompleted();
    fn Cancel();
    fn Pause();
    fn GetRowStatus();
    fn GetColumnStatus();
}
pub trait IITWordWheelImpl: Sized {
    fn Open();
    fn Close();
    fn GetLocaleInfo();
    fn GetSorterInstance();
    fn Count();
    fn Lookup();
    fn Lookup();
    fn Lookup();
    fn SetGroup();
    fn GetGroup();
    fn GetDataCount();
    fn GetData();
    fn GetDataColumns();
}
pub trait IStemSinkImpl: Sized {
    fn PutAltWord();
    fn PutWord();
}
pub trait IStemmerConfigImpl: Sized {
    fn SetLocaleInfo();
    fn GetLocaleInfo();
    fn SetControlInfo();
    fn GetControlInfo();
    fn LoadExternalStemmerData();
}
pub trait IWordBreakerConfigImpl: Sized {
    fn SetLocaleInfo();
    fn GetLocaleInfo();
    fn SetBreakWordType();
    fn GetBreakWordType();
    fn SetControlInfo();
    fn GetControlInfo();
    fn LoadExternalBreakerData();
    fn SetWordStemmer();
    fn GetWordStemmer();
}
