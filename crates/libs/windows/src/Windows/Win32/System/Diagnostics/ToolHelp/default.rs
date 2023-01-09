impl ::core::default::Default for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_TOOLHELP_SNAPSHOT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CREATE_TOOLHELP_SNAPSHOT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HEAPENTRY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HEAPENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.hHandle == other.hHandle && self.dwAddress == other.dwAddress && self.dwBlockSize == other.dwBlockSize && self.dwFlags == other.dwFlags && self.dwLockCount == other.dwLockCount && self.dwResvd == other.dwResvd && self.th32ProcessID == other.th32ProcessID && self.th32HeapID == other.th32HeapID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HEAPENTRY32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HEAPENTRY32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HEAPENTRY32").field("dwSize", &self.dwSize).field("hHandle", &self.hHandle).field("dwAddress", &self.dwAddress).field("dwBlockSize", &self.dwBlockSize).field("dwFlags", &self.dwFlags).field("dwLockCount", &self.dwLockCount).field("dwResvd", &self.dwResvd).field("th32ProcessID", &self.th32ProcessID).field("th32HeapID", &self.th32HeapID).finish()
    }
}
impl ::core::default::Default for HEAPENTRY32_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HEAPENTRY32_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HEAPENTRY32_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for HEAPLIST32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HEAPLIST32 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.th32ProcessID == other.th32ProcessID && self.th32HeapID == other.th32HeapID && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for HEAPLIST32 {}
impl ::core::fmt::Debug for HEAPLIST32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HEAPLIST32").field("dwSize", &self.dwSize).field("th32ProcessID", &self.th32ProcessID).field("th32HeapID", &self.th32HeapID).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MODULEENTRY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MODULEENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.th32ModuleID == other.th32ModuleID && self.th32ProcessID == other.th32ProcessID && self.GlblcntUsage == other.GlblcntUsage && self.ProccntUsage == other.ProccntUsage && self.modBaseAddr == other.modBaseAddr && self.modBaseSize == other.modBaseSize && self.hModule == other.hModule && self.szModule == other.szModule && self.szExePath == other.szExePath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MODULEENTRY32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MODULEENTRY32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MODULEENTRY32").field("dwSize", &self.dwSize).field("th32ModuleID", &self.th32ModuleID).field("th32ProcessID", &self.th32ProcessID).field("GlblcntUsage", &self.GlblcntUsage).field("ProccntUsage", &self.ProccntUsage).field("modBaseAddr", &self.modBaseAddr).field("modBaseSize", &self.modBaseSize).field("hModule", &self.hModule).field("szModule", &self.szModule).field("szExePath", &self.szExePath).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MODULEENTRY32W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MODULEENTRY32W {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.th32ModuleID == other.th32ModuleID && self.th32ProcessID == other.th32ProcessID && self.GlblcntUsage == other.GlblcntUsage && self.ProccntUsage == other.ProccntUsage && self.modBaseAddr == other.modBaseAddr && self.modBaseSize == other.modBaseSize && self.hModule == other.hModule && self.szModule == other.szModule && self.szExePath == other.szExePath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MODULEENTRY32W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MODULEENTRY32W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MODULEENTRY32W").field("dwSize", &self.dwSize).field("th32ModuleID", &self.th32ModuleID).field("th32ProcessID", &self.th32ProcessID).field("GlblcntUsage", &self.GlblcntUsage).field("ProccntUsage", &self.ProccntUsage).field("modBaseAddr", &self.modBaseAddr).field("modBaseSize", &self.modBaseSize).field("hModule", &self.hModule).field("szModule", &self.szModule).field("szExePath", &self.szExePath).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROCESSENTRY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROCESSENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.cntUsage == other.cntUsage && self.th32ProcessID == other.th32ProcessID && self.th32DefaultHeapID == other.th32DefaultHeapID && self.th32ModuleID == other.th32ModuleID && self.cntThreads == other.cntThreads && self.th32ParentProcessID == other.th32ParentProcessID && self.pcPriClassBase == other.pcPriClassBase && self.dwFlags == other.dwFlags && self.szExeFile == other.szExeFile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROCESSENTRY32 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PROCESSENTRY32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESSENTRY32").field("dwSize", &self.dwSize).field("cntUsage", &self.cntUsage).field("th32ProcessID", &self.th32ProcessID).field("th32DefaultHeapID", &self.th32DefaultHeapID).field("th32ModuleID", &self.th32ModuleID).field("cntThreads", &self.cntThreads).field("th32ParentProcessID", &self.th32ParentProcessID).field("pcPriClassBase", &self.pcPriClassBase).field("dwFlags", &self.dwFlags).field("szExeFile", &self.szExeFile).finish()
    }
}
impl ::core::default::Default for PROCESSENTRY32W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PROCESSENTRY32W {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.cntUsage == other.cntUsage && self.th32ProcessID == other.th32ProcessID && self.th32DefaultHeapID == other.th32DefaultHeapID && self.th32ModuleID == other.th32ModuleID && self.cntThreads == other.cntThreads && self.th32ParentProcessID == other.th32ParentProcessID && self.pcPriClassBase == other.pcPriClassBase && self.dwFlags == other.dwFlags && self.szExeFile == other.szExeFile
    }
}
impl ::core::cmp::Eq for PROCESSENTRY32W {}
impl ::core::fmt::Debug for PROCESSENTRY32W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESSENTRY32W").field("dwSize", &self.dwSize).field("cntUsage", &self.cntUsage).field("th32ProcessID", &self.th32ProcessID).field("th32DefaultHeapID", &self.th32DefaultHeapID).field("th32ModuleID", &self.th32ModuleID).field("cntThreads", &self.cntThreads).field("th32ParentProcessID", &self.th32ParentProcessID).field("pcPriClassBase", &self.pcPriClassBase).field("dwFlags", &self.dwFlags).field("szExeFile", &self.szExeFile).finish()
    }
}
impl ::core::default::Default for THREADENTRY32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for THREADENTRY32 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.cntUsage == other.cntUsage && self.th32ThreadID == other.th32ThreadID && self.th32OwnerProcessID == other.th32OwnerProcessID && self.tpBasePri == other.tpBasePri && self.tpDeltaPri == other.tpDeltaPri && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for THREADENTRY32 {}
impl ::core::fmt::Debug for THREADENTRY32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("THREADENTRY32").field("dwSize", &self.dwSize).field("cntUsage", &self.cntUsage).field("th32ThreadID", &self.th32ThreadID).field("th32OwnerProcessID", &self.th32OwnerProcessID).field("tpBasePri", &self.tpBasePri).field("tpDeltaPri", &self.tpDeltaPri).field("dwFlags", &self.dwFlags).finish()
    }
}
