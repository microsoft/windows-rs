#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CCAB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CCAB {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.cbFolderThresh == other.cbFolderThresh && self.cbReserveCFHeader == other.cbReserveCFHeader && self.cbReserveCFFolder == other.cbReserveCFFolder && self.cbReserveCFData == other.cbReserveCFData && self.iCab == other.iCab && self.iDisk == other.iDisk && self.fFailOnIncompressible == other.fFailOnIncompressible && self.setID == other.setID && self.szDisk == other.szDisk && self.szCab == other.szCab && self.szCabPath == other.szCabPath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CCAB {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CCAB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CCAB")
            .field("cb", &self.cb)
            .field("cbFolderThresh", &self.cbFolderThresh)
            .field("cbReserveCFHeader", &self.cbReserveCFHeader)
            .field("cbReserveCFFolder", &self.cbReserveCFFolder)
            .field("cbReserveCFData", &self.cbReserveCFData)
            .field("iCab", &self.iCab)
            .field("iDisk", &self.iDisk)
            .field("fFailOnIncompressible", &self.fFailOnIncompressible)
            .field("setID", &self.setID)
            .field("szDisk", &self.szDisk)
            .field("szCab", &self.szCab)
            .field("szCabPath", &self.szCabPath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ERF {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ERF {
    fn eq(&self, other: &Self) -> bool {
        self.erfOper == other.erfOper && self.erfType == other.erfType && self.fError == other.fError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ERF {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ERF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ERF").field("erfOper", &self.erfOper).field("erfType", &self.erfType).field("fError", &self.fError).finish()
    }
}
impl ::core::default::Default for FCIERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FCIERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FCIERROR").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FDICABINETINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FDICABINETINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbCabinet == other.cbCabinet && self.cFolders == other.cFolders && self.cFiles == other.cFiles && self.setID == other.setID && self.iCabinet == other.iCabinet && self.fReserve == other.fReserve && self.hasprev == other.hasprev && self.hasnext == other.hasnext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FDICABINETINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FDICABINETINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FDICABINETINFO").field("cbCabinet", &self.cbCabinet).field("cFolders", &self.cFolders).field("cFiles", &self.cFiles).field("setID", &self.setID).field("iCabinet", &self.iCabinet).field("fReserve", &self.fReserve).field("hasprev", &self.hasprev).field("hasnext", &self.hasnext).finish()
    }
}
impl ::core::default::Default for FDICREATE_CPU_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FDICREATE_CPU_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FDICREATE_CPU_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FDIDECRYPT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FDIDECRYPTTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FDIDECRYPTTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FDIDECRYPTTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FDIERROR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FDIERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FDIERROR").field(&self.0).finish()
    }
}
impl ::core::default::Default for FDINOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FDINOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.cb == other.cb && self.psz1 == other.psz1 && self.psz2 == other.psz2 && self.psz3 == other.psz3 && self.pv == other.pv && self.hf == other.hf && self.date == other.date && self.time == other.time && self.attribs == other.attribs && self.setID == other.setID && self.iCabinet == other.iCabinet && self.iFolder == other.iFolder && self.fdie == other.fdie
    }
}
impl ::core::cmp::Eq for FDINOTIFICATION {}
impl ::core::fmt::Debug for FDINOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FDINOTIFICATION").field("cb", &self.cb).field("psz1", &self.psz1).field("psz2", &self.psz2).field("psz3", &self.psz3).field("pv", &self.pv).field("hf", &self.hf).field("date", &self.date).field("time", &self.time).field("attribs", &self.attribs).field("setID", &self.setID).field("iCabinet", &self.iCabinet).field("iFolder", &self.iFolder).field("fdie", &self.fdie).finish()
    }
}
impl ::core::default::Default for FDINOTIFICATIONTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FDINOTIFICATIONTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FDINOTIFICATIONTYPE").field(&self.0).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FDISPILLFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FDISPILLFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
