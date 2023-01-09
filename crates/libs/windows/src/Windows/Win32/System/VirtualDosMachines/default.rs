#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for GLOBALENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IMAGE_NOTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IMAGE_NOTE {
    fn eq(&self, other: &Self) -> bool {
        self.Module == other.Module && self.FileName == other.FileName && self.hModule == other.hModule && self.hTask == other.hTask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IMAGE_NOTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IMAGE_NOTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IMAGE_NOTE").field("Module", &self.Module).field("FileName", &self.FileName).field("hModule", &self.hModule).field("hTask", &self.hTask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MODULEENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SEGMENT_NOTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SEGMENT_NOTE {
    fn eq(&self, other: &Self) -> bool {
        self.Selector1 == other.Selector1 && self.Selector2 == other.Selector2 && self.Segment == other.Segment && self.Module == other.Module && self.FileName == other.FileName && self.Type == other.Type && self.Length == other.Length
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SEGMENT_NOTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SEGMENT_NOTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEGMENT_NOTE").field("Selector1", &self.Selector1).field("Selector2", &self.Selector2).field("Segment", &self.Segment).field("Module", &self.Module).field("FileName", &self.FileName).field("Type", &self.Type).field("Length", &self.Length).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TEMP_BP_NOTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TEMP_BP_NOTE {
    fn eq(&self, other: &Self) -> bool {
        self.Seg == other.Seg && self.Offset == other.Offset && self.bPM == other.bPM
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TEMP_BP_NOTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TEMP_BP_NOTE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TEMP_BP_NOTE").field("Seg", &self.Seg).field("Offset", &self.Offset).field("bPM", &self.bPM).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for VDMCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::PartialEq for VDMCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ContextFlags == other.ContextFlags && self.Dr0 == other.Dr0 && self.Dr1 == other.Dr1 && self.Dr2 == other.Dr2 && self.Dr3 == other.Dr3 && self.Dr6 == other.Dr6 && self.Dr7 == other.Dr7 && self.FloatSave == other.FloatSave && self.SegGs == other.SegGs && self.SegFs == other.SegFs && self.SegEs == other.SegEs && self.SegDs == other.SegDs && self.Edi == other.Edi && self.Esi == other.Esi && self.Ebx == other.Ebx && self.Edx == other.Edx && self.Ecx == other.Ecx && self.Eax == other.Eax && self.Ebp == other.Ebp && self.Eip == other.Eip && self.SegCs == other.SegCs && self.EFlags == other.EFlags && self.Esp == other.Esp && self.SegSs == other.SegSs && self.ExtendedRegisters == other.ExtendedRegisters
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::Eq for VDMCONTEXT {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::fmt::Debug for VDMCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDMCONTEXT")
            .field("ContextFlags", &self.ContextFlags)
            .field("Dr0", &self.Dr0)
            .field("Dr1", &self.Dr1)
            .field("Dr2", &self.Dr2)
            .field("Dr3", &self.Dr3)
            .field("Dr6", &self.Dr6)
            .field("Dr7", &self.Dr7)
            .field("FloatSave", &self.FloatSave)
            .field("SegGs", &self.SegGs)
            .field("SegFs", &self.SegFs)
            .field("SegEs", &self.SegEs)
            .field("SegDs", &self.SegDs)
            .field("Edi", &self.Edi)
            .field("Esi", &self.Esi)
            .field("Ebx", &self.Ebx)
            .field("Edx", &self.Edx)
            .field("Ecx", &self.Ecx)
            .field("Eax", &self.Eax)
            .field("Ebp", &self.Ebp)
            .field("Eip", &self.Eip)
            .field("SegCs", &self.SegCs)
            .field("EFlags", &self.EFlags)
            .field("Esp", &self.Esp)
            .field("SegSs", &self.SegSs)
            .field("ExtendedRegisters", &self.ExtendedRegisters)
            .finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for VDMCONTEXT_WITHOUT_XSAVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::PartialEq for VDMCONTEXT_WITHOUT_XSAVE {
    fn eq(&self, other: &Self) -> bool {
        self.ContextFlags == other.ContextFlags && self.Dr0 == other.Dr0 && self.Dr1 == other.Dr1 && self.Dr2 == other.Dr2 && self.Dr3 == other.Dr3 && self.Dr6 == other.Dr6 && self.Dr7 == other.Dr7 && self.FloatSave == other.FloatSave && self.SegGs == other.SegGs && self.SegFs == other.SegFs && self.SegEs == other.SegEs && self.SegDs == other.SegDs && self.Edi == other.Edi && self.Esi == other.Esi && self.Ebx == other.Ebx && self.Edx == other.Edx && self.Ecx == other.Ecx && self.Eax == other.Eax && self.Ebp == other.Ebp && self.Eip == other.Eip && self.SegCs == other.SegCs && self.EFlags == other.EFlags && self.Esp == other.Esp && self.SegSs == other.SegSs
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::Eq for VDMCONTEXT_WITHOUT_XSAVE {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::fmt::Debug for VDMCONTEXT_WITHOUT_XSAVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDMCONTEXT_WITHOUT_XSAVE")
            .field("ContextFlags", &self.ContextFlags)
            .field("Dr0", &self.Dr0)
            .field("Dr1", &self.Dr1)
            .field("Dr2", &self.Dr2)
            .field("Dr3", &self.Dr3)
            .field("Dr6", &self.Dr6)
            .field("Dr7", &self.Dr7)
            .field("FloatSave", &self.FloatSave)
            .field("SegGs", &self.SegGs)
            .field("SegFs", &self.SegFs)
            .field("SegEs", &self.SegEs)
            .field("SegDs", &self.SegDs)
            .field("Edi", &self.Edi)
            .field("Esi", &self.Esi)
            .field("Ebx", &self.Ebx)
            .field("Edx", &self.Edx)
            .field("Ecx", &self.Ecx)
            .field("Eax", &self.Eax)
            .field("Ebp", &self.Ebp)
            .field("Eip", &self.Eip)
            .field("SegCs", &self.SegCs)
            .field("EFlags", &self.EFlags)
            .field("Esp", &self.Esp)
            .field("SegSs", &self.SegSs)
            .finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for VDMLDT_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VDM_SEGINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VDM_SEGINFO {
    fn eq(&self, other: &Self) -> bool {
        self.Selector == other.Selector && self.SegNumber == other.SegNumber && self.Length == other.Length && self.Type == other.Type && self.ModuleName == other.ModuleName && self.FileName == other.FileName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VDM_SEGINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VDM_SEGINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VDM_SEGINFO").field("Selector", &self.Selector).field("SegNumber", &self.SegNumber).field("Length", &self.Length).field("Type", &self.Type).field("ModuleName", &self.ModuleName).field("FileName", &self.FileName).finish()
    }
}
