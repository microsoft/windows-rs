impl ::core::default::Default for MapiFileDesc {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MapiFileDesc {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved && self.flFlags == other.flFlags && self.nPosition == other.nPosition && self.lpszPathName == other.lpszPathName && self.lpszFileName == other.lpszFileName && self.lpFileType == other.lpFileType
    }
}
impl ::core::cmp::Eq for MapiFileDesc {}
impl ::core::fmt::Debug for MapiFileDesc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MapiFileDesc").field("ulReserved", &self.ulReserved).field("flFlags", &self.flFlags).field("nPosition", &self.nPosition).field("lpszPathName", &self.lpszPathName).field("lpszFileName", &self.lpszFileName).field("lpFileType", &self.lpFileType).finish()
    }
}
impl ::core::default::Default for MapiFileDescW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MapiFileDescW {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved && self.flFlags == other.flFlags && self.nPosition == other.nPosition && self.lpszPathName == other.lpszPathName && self.lpszFileName == other.lpszFileName && self.lpFileType == other.lpFileType
    }
}
impl ::core::cmp::Eq for MapiFileDescW {}
impl ::core::fmt::Debug for MapiFileDescW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MapiFileDescW").field("ulReserved", &self.ulReserved).field("flFlags", &self.flFlags).field("nPosition", &self.nPosition).field("lpszPathName", &self.lpszPathName).field("lpszFileName", &self.lpszFileName).field("lpFileType", &self.lpFileType).finish()
    }
}
impl ::core::default::Default for MapiFileTagExt {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MapiFileTagExt {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved && self.cbTag == other.cbTag && self.lpTag == other.lpTag && self.cbEncoding == other.cbEncoding && self.lpEncoding == other.lpEncoding
    }
}
impl ::core::cmp::Eq for MapiFileTagExt {}
impl ::core::fmt::Debug for MapiFileTagExt {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MapiFileTagExt").field("ulReserved", &self.ulReserved).field("cbTag", &self.cbTag).field("lpTag", &self.lpTag).field("cbEncoding", &self.cbEncoding).field("lpEncoding", &self.lpEncoding).finish()
    }
}
impl ::core::default::Default for MapiMessage {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MapiMessage {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved && self.lpszSubject == other.lpszSubject && self.lpszNoteText == other.lpszNoteText && self.lpszMessageType == other.lpszMessageType && self.lpszDateReceived == other.lpszDateReceived && self.lpszConversationID == other.lpszConversationID && self.flFlags == other.flFlags && self.lpOriginator == other.lpOriginator && self.nRecipCount == other.nRecipCount && self.lpRecips == other.lpRecips && self.nFileCount == other.nFileCount && self.lpFiles == other.lpFiles
    }
}
impl ::core::cmp::Eq for MapiMessage {}
impl ::core::fmt::Debug for MapiMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MapiMessage")
            .field("ulReserved", &self.ulReserved)
            .field("lpszSubject", &self.lpszSubject)
            .field("lpszNoteText", &self.lpszNoteText)
            .field("lpszMessageType", &self.lpszMessageType)
            .field("lpszDateReceived", &self.lpszDateReceived)
            .field("lpszConversationID", &self.lpszConversationID)
            .field("flFlags", &self.flFlags)
            .field("lpOriginator", &self.lpOriginator)
            .field("nRecipCount", &self.nRecipCount)
            .field("lpRecips", &self.lpRecips)
            .field("nFileCount", &self.nFileCount)
            .field("lpFiles", &self.lpFiles)
            .finish()
    }
}
impl ::core::default::Default for MapiMessageW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MapiMessageW {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved && self.lpszSubject == other.lpszSubject && self.lpszNoteText == other.lpszNoteText && self.lpszMessageType == other.lpszMessageType && self.lpszDateReceived == other.lpszDateReceived && self.lpszConversationID == other.lpszConversationID && self.flFlags == other.flFlags && self.lpOriginator == other.lpOriginator && self.nRecipCount == other.nRecipCount && self.lpRecips == other.lpRecips && self.nFileCount == other.nFileCount && self.lpFiles == other.lpFiles
    }
}
impl ::core::cmp::Eq for MapiMessageW {}
impl ::core::fmt::Debug for MapiMessageW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MapiMessageW")
            .field("ulReserved", &self.ulReserved)
            .field("lpszSubject", &self.lpszSubject)
            .field("lpszNoteText", &self.lpszNoteText)
            .field("lpszMessageType", &self.lpszMessageType)
            .field("lpszDateReceived", &self.lpszDateReceived)
            .field("lpszConversationID", &self.lpszConversationID)
            .field("flFlags", &self.flFlags)
            .field("lpOriginator", &self.lpOriginator)
            .field("nRecipCount", &self.nRecipCount)
            .field("lpRecips", &self.lpRecips)
            .field("nFileCount", &self.nFileCount)
            .field("lpFiles", &self.lpFiles)
            .finish()
    }
}
impl ::core::default::Default for MapiRecipDesc {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MapiRecipDesc {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved && self.ulRecipClass == other.ulRecipClass && self.lpszName == other.lpszName && self.lpszAddress == other.lpszAddress && self.ulEIDSize == other.ulEIDSize && self.lpEntryID == other.lpEntryID
    }
}
impl ::core::cmp::Eq for MapiRecipDesc {}
impl ::core::fmt::Debug for MapiRecipDesc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MapiRecipDesc").field("ulReserved", &self.ulReserved).field("ulRecipClass", &self.ulRecipClass).field("lpszName", &self.lpszName).field("lpszAddress", &self.lpszAddress).field("ulEIDSize", &self.ulEIDSize).field("lpEntryID", &self.lpEntryID).finish()
    }
}
impl ::core::default::Default for MapiRecipDescW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MapiRecipDescW {
    fn eq(&self, other: &Self) -> bool {
        self.ulReserved == other.ulReserved && self.ulRecipClass == other.ulRecipClass && self.lpszName == other.lpszName && self.lpszAddress == other.lpszAddress && self.ulEIDSize == other.ulEIDSize && self.lpEntryID == other.lpEntryID
    }
}
impl ::core::cmp::Eq for MapiRecipDescW {}
impl ::core::fmt::Debug for MapiRecipDescW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MapiRecipDescW").field("ulReserved", &self.ulReserved).field("ulRecipClass", &self.ulRecipClass).field("lpszName", &self.lpszName).field("lpszAddress", &self.lpszAddress).field("ulEIDSize", &self.ulEIDSize).field("lpEntryID", &self.lpEntryID).finish()
    }
}
