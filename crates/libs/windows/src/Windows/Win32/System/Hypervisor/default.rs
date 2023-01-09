impl ::core::default::Default for DOS_IMAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOS_IMAGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.PdbName == other.PdbName && self.ImageBaseAddress == other.ImageBaseAddress && self.ImageSize == other.ImageSize && self.Timestamp == other.Timestamp
    }
}
impl ::core::cmp::Eq for DOS_IMAGE_INFO {}
impl ::core::fmt::Debug for DOS_IMAGE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOS_IMAGE_INFO").field("PdbName", &self.PdbName).field("ImageBaseAddress", &self.ImageBaseAddress).field("ImageSize", &self.ImageSize).field("Timestamp", &self.Timestamp).finish()
    }
}
impl ::core::default::Default for GPA_MEMORY_CHUNK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GPA_MEMORY_CHUNK {
    fn eq(&self, other: &Self) -> bool {
        self.GuestPhysicalStartPageIndex == other.GuestPhysicalStartPageIndex && self.PageCount == other.PageCount
    }
}
impl ::core::cmp::Eq for GPA_MEMORY_CHUNK {}
impl ::core::fmt::Debug for GPA_MEMORY_CHUNK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GPA_MEMORY_CHUNK").field("GuestPhysicalStartPageIndex", &self.GuestPhysicalStartPageIndex).field("PageCount", &self.PageCount).finish()
    }
}
impl ::core::default::Default for GUEST_OS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for GUEST_OS_MICROSOFT_IDS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GUEST_OS_MICROSOFT_IDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GUEST_OS_MICROSOFT_IDS").field(&self.0).finish()
    }
}
impl ::core::default::Default for GUEST_OS_OPENSOURCE_IDS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GUEST_OS_OPENSOURCE_IDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GUEST_OS_OPENSOURCE_IDS").field(&self.0).finish()
    }
}
impl ::core::default::Default for GUEST_OS_VENDOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GUEST_OS_VENDOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GUEST_OS_VENDOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for HDV_DEVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HDV_DEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDV_DEVICE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for HDV_DOORBELL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HDV_DOORBELL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDV_DOORBELL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for HDV_MMIO_MAPPING_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HDV_MMIO_MAPPING_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDV_MMIO_MAPPING_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HDV_MMIO_MAPPING_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HDV_MMIO_MAPPING_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HDV_MMIO_MAPPING_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HDV_MMIO_MAPPING_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HDV_MMIO_MAPPING_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for HDV_PCI_BAR_SELECTOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HDV_PCI_BAR_SELECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDV_PCI_BAR_SELECTOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for HDV_PCI_DEVICE_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HDV_PCI_INTERFACE_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for HDV_PCI_INTERFACE_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HDV_PCI_INTERFACE_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for HDV_PCI_PNP_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HDV_PCI_PNP_ID {
    fn eq(&self, other: &Self) -> bool {
        self.VendorID == other.VendorID && self.DeviceID == other.DeviceID && self.RevisionID == other.RevisionID && self.ProgIf == other.ProgIf && self.SubClass == other.SubClass && self.BaseClass == other.BaseClass && self.SubVendorID == other.SubVendorID && self.SubSystemID == other.SubSystemID
    }
}
impl ::core::cmp::Eq for HDV_PCI_PNP_ID {}
impl ::core::fmt::Debug for HDV_PCI_PNP_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HDV_PCI_PNP_ID").field("VendorID", &self.VendorID).field("DeviceID", &self.DeviceID).field("RevisionID", &self.RevisionID).field("ProgIf", &self.ProgIf).field("SubClass", &self.SubClass).field("BaseClass", &self.BaseClass).field("SubVendorID", &self.SubVendorID).field("SubSystemID", &self.SubSystemID).finish()
    }
}
impl ::core::default::Default for HVSOCKET_ADDRESS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HVSOCKET_ADDRESS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SystemId == other.SystemId && self.VirtualMachineId == other.VirtualMachineId && self.SiloId == other.SiloId && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for HVSOCKET_ADDRESS_INFO {}
impl ::core::fmt::Debug for HVSOCKET_ADDRESS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HVSOCKET_ADDRESS_INFO").field("SystemId", &self.SystemId).field("VirtualMachineId", &self.VirtualMachineId).field("SiloId", &self.SiloId).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for MODULE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MODULE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessImageName == other.ProcessImageName && self.Image == other.Image
    }
}
impl ::core::cmp::Eq for MODULE_INFO {}
impl ::core::fmt::Debug for MODULE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MODULE_INFO").field("ProcessImageName", &self.ProcessImageName).field("Image", &self.Image).finish()
    }
}
impl ::core::default::Default for PAGING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PAGING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAGING_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for REGISTER_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REGISTER_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REGISTER_ID").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for SOCKADDR_HV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for SOCKADDR_HV {
    fn eq(&self, other: &Self) -> bool {
        self.Family == other.Family && self.Reserved == other.Reserved && self.VmId == other.VmId && self.ServiceId == other.ServiceId
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for SOCKADDR_HV {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for SOCKADDR_HV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOCKADDR_HV").field("Family", &self.Family).field("Reserved", &self.Reserved).field("VmId", &self.VmId).field("ServiceId", &self.ServiceId).finish()
    }
}
impl ::core::default::Default for VIRTUAL_PROCESSOR_ARCH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VIRTUAL_PROCESSOR_ARCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIRTUAL_PROCESSOR_ARCH").field(&self.0).finish()
    }
}
impl ::core::default::Default for VIRTUAL_PROCESSOR_REGISTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for VIRTUAL_PROCESSOR_VENDOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VIRTUAL_PROCESSOR_VENDOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIRTUAL_PROCESSOR_VENDOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for VM_GENCOUNTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VM_GENCOUNTER {
    fn eq(&self, other: &Self) -> bool {
        self.GenerationCount == other.GenerationCount && self.GenerationCountHigh == other.GenerationCountHigh
    }
}
impl ::core::cmp::Eq for VM_GENCOUNTER {}
impl ::core::fmt::Debug for VM_GENCOUNTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VM_GENCOUNTER").field("GenerationCount", &self.GenerationCount).field("GenerationCountHigh", &self.GenerationCountHigh).finish()
    }
}
impl ::core::default::Default for WHV_ACCESS_GPA_CONTROLS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_ADVISE_GPA_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_ADVISE_GPA_RANGE_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_ADVISE_GPA_RANGE_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_ADVISE_GPA_RANGE_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_ADVISE_GPA_RANGE_POPULATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_ALLOCATE_VPCI_RESOURCE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_ALLOCATE_VPCI_RESOURCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_ALLOCATE_VPCI_RESOURCE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WHV_ALLOCATE_VPCI_RESOURCE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WHV_ALLOCATE_VPCI_RESOURCE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WHV_ALLOCATE_VPCI_RESOURCE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WHV_ALLOCATE_VPCI_RESOURCE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WHV_ALLOCATE_VPCI_RESOURCE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for WHV_CACHE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_CACHE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_CACHE_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHV_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_CAPABILITY_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_CAPABILITY_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_CAPABILITY_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_CAPABILITY_FEATURES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.HighestFrequencyMhz == other.HighestFrequencyMhz && self.NominalFrequencyMhz == other.NominalFrequencyMhz && self.LowestFrequencyMhz == other.LowestFrequencyMhz && self.FrequencyStepMhz == other.FrequencyStepMhz
    }
}
impl ::core::cmp::Eq for WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP {}
impl ::core::fmt::Debug for WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP").field("_bitfield", &self._bitfield).field("HighestFrequencyMhz", &self.HighestFrequencyMhz).field("NominalFrequencyMhz", &self.NominalFrequencyMhz).field("LowestFrequencyMhz", &self.LowestFrequencyMhz).field("FrequencyStepMhz", &self.FrequencyStepMhz).finish()
    }
}
impl ::core::default::Default for WHV_CPUID_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_CPUID_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Eax == other.Eax && self.Ebx == other.Ebx && self.Ecx == other.Ecx && self.Edx == other.Edx
    }
}
impl ::core::cmp::Eq for WHV_CPUID_OUTPUT {}
impl ::core::fmt::Debug for WHV_CPUID_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_CPUID_OUTPUT").field("Eax", &self.Eax).field("Ebx", &self.Ebx).field("Ecx", &self.Ecx).field("Edx", &self.Edx).finish()
    }
}
impl ::core::default::Default for WHV_CREATE_VPCI_DEVICE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_CREATE_VPCI_DEVICE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_CREATE_VPCI_DEVICE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WHV_CREATE_VPCI_DEVICE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WHV_CREATE_VPCI_DEVICE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WHV_CREATE_VPCI_DEVICE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WHV_CREATE_VPCI_DEVICE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WHV_CREATE_VPCI_DEVICE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for WHV_DOORBELL_MATCH_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_DOORBELL_MATCH_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.GuestAddress == other.GuestAddress && self.Value == other.Value && self.Length == other.Length && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for WHV_DOORBELL_MATCH_DATA {}
impl ::core::fmt::Debug for WHV_DOORBELL_MATCH_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_DOORBELL_MATCH_DATA").field("GuestAddress", &self.GuestAddress).field("Value", &self.Value).field("Length", &self.Length).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for WHV_EMULATOR_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_EMULATOR_IO_ACCESS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_EMULATOR_IO_ACCESS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Direction == other.Direction && self.Port == other.Port && self.AccessSize == other.AccessSize && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for WHV_EMULATOR_IO_ACCESS_INFO {}
impl ::core::fmt::Debug for WHV_EMULATOR_IO_ACCESS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_EMULATOR_IO_ACCESS_INFO").field("Direction", &self.Direction).field("Port", &self.Port).field("AccessSize", &self.AccessSize).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for WHV_EMULATOR_MEMORY_ACCESS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_EMULATOR_MEMORY_ACCESS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.GpaAddress == other.GpaAddress && self.Direction == other.Direction && self.AccessSize == other.AccessSize && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for WHV_EMULATOR_MEMORY_ACCESS_INFO {}
impl ::core::fmt::Debug for WHV_EMULATOR_MEMORY_ACCESS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_EMULATOR_MEMORY_ACCESS_INFO").field("GpaAddress", &self.GpaAddress).field("Direction", &self.Direction).field("AccessSize", &self.AccessSize).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for WHV_EMULATOR_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_EXCEPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_EXCEPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_EXCEPTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_EXTENDED_VM_EXITS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_HYPERCALL_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_INTERNAL_ACTIVITY_REGISTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_INTERRUPT_CONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_INTERRUPT_CONTROL {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield && self.Destination == other.Destination && self.Vector == other.Vector
    }
}
impl ::core::cmp::Eq for WHV_INTERRUPT_CONTROL {}
impl ::core::fmt::Debug for WHV_INTERRUPT_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_INTERRUPT_CONTROL").field("_bitfield", &self._bitfield).field("Destination", &self.Destination).field("Vector", &self.Vector).finish()
    }
}
impl ::core::default::Default for WHV_INTERRUPT_DESTINATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_INTERRUPT_DESTINATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_INTERRUPT_DESTINATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_INTERRUPT_TRIGGER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_INTERRUPT_TRIGGER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_INTERRUPT_TRIGGER_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_INTERRUPT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_INTERRUPT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_INTERRUPT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_MAP_GPA_RANGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_MAP_GPA_RANGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_MAP_GPA_RANGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WHV_MAP_GPA_RANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WHV_MAP_GPA_RANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WHV_MAP_GPA_RANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WHV_MAP_GPA_RANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WHV_MAP_GPA_RANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for WHV_MEMORY_ACCESS_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_MEMORY_ACCESS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_MEMORY_ACCESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_MEMORY_ACCESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_MEMORY_ACCESS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_MEMORY_RANGE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_MEMORY_RANGE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.GuestAddress == other.GuestAddress && self.SizeInBytes == other.SizeInBytes
    }
}
impl ::core::cmp::Eq for WHV_MEMORY_RANGE_ENTRY {}
impl ::core::fmt::Debug for WHV_MEMORY_RANGE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_MEMORY_RANGE_ENTRY").field("GuestAddress", &self.GuestAddress).field("SizeInBytes", &self.SizeInBytes).finish()
    }
}
impl ::core::default::Default for WHV_MSR_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_MSR_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_MSR_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_MSR_ACTION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_MSR_ACTION_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Index == other.Index && self.ReadAction == other.ReadAction && self.WriteAction == other.WriteAction && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for WHV_MSR_ACTION_ENTRY {}
impl ::core::fmt::Debug for WHV_MSR_ACTION_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_MSR_ACTION_ENTRY").field("Index", &self.Index).field("ReadAction", &self.ReadAction).field("WriteAction", &self.WriteAction).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for WHV_NOTIFICATION_PORT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_NOTIFICATION_PORT_PROPERTY_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_NOTIFICATION_PORT_PROPERTY_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_NOTIFICATION_PORT_PROPERTY_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_NOTIFICATION_PORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_NOTIFICATION_PORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_NOTIFICATION_PORT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_PARTITION_COUNTER_SET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_PARTITION_COUNTER_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_PARTITION_COUNTER_SET").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_PARTITION_MEMORY_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_PARTITION_MEMORY_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.Mapped4KPageCount == other.Mapped4KPageCount && self.Mapped2MPageCount == other.Mapped2MPageCount && self.Mapped1GPageCount == other.Mapped1GPageCount
    }
}
impl ::core::cmp::Eq for WHV_PARTITION_MEMORY_COUNTERS {}
impl ::core::fmt::Debug for WHV_PARTITION_MEMORY_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_PARTITION_MEMORY_COUNTERS").field("Mapped4KPageCount", &self.Mapped4KPageCount).field("Mapped2MPageCount", &self.Mapped2MPageCount).field("Mapped1GPageCount", &self.Mapped1GPageCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHV_PARTITION_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_PARTITION_PROPERTY_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_PARTITION_PROPERTY_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_PARTITION_PROPERTY_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_PROCESSOR_APIC_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_PROCESSOR_APIC_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.MmioAccessCount == other.MmioAccessCount && self.EoiAccessCount == other.EoiAccessCount && self.TprAccessCount == other.TprAccessCount && self.SentIpiCount == other.SentIpiCount && self.SelfIpiCount == other.SelfIpiCount
    }
}
impl ::core::cmp::Eq for WHV_PROCESSOR_APIC_COUNTERS {}
impl ::core::fmt::Debug for WHV_PROCESSOR_APIC_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_PROCESSOR_APIC_COUNTERS").field("MmioAccessCount", &self.MmioAccessCount).field("EoiAccessCount", &self.EoiAccessCount).field("TprAccessCount", &self.TprAccessCount).field("SentIpiCount", &self.SentIpiCount).field("SelfIpiCount", &self.SelfIpiCount).finish()
    }
}
impl ::core::default::Default for WHV_PROCESSOR_COUNTER_SET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_PROCESSOR_COUNTER_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_PROCESSOR_COUNTER_SET").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_PROCESSOR_EVENT_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_PROCESSOR_EVENT_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.PageFaultCount == other.PageFaultCount && self.ExceptionCount == other.ExceptionCount && self.InterruptCount == other.InterruptCount
    }
}
impl ::core::cmp::Eq for WHV_PROCESSOR_EVENT_COUNTERS {}
impl ::core::fmt::Debug for WHV_PROCESSOR_EVENT_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_PROCESSOR_EVENT_COUNTERS").field("PageFaultCount", &self.PageFaultCount).field("ExceptionCount", &self.ExceptionCount).field("InterruptCount", &self.InterruptCount).finish()
    }
}
impl ::core::default::Default for WHV_PROCESSOR_FEATURES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_PROCESSOR_FEATURES1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_PROCESSOR_FEATURES_BANKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_PROCESSOR_INTERCEPT_COUNTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_PROCESSOR_INTERCEPT_COUNTER {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Time100ns == other.Time100ns
    }
}
impl ::core::cmp::Eq for WHV_PROCESSOR_INTERCEPT_COUNTER {}
impl ::core::fmt::Debug for WHV_PROCESSOR_INTERCEPT_COUNTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_PROCESSOR_INTERCEPT_COUNTER").field("Count", &self.Count).field("Time100ns", &self.Time100ns).finish()
    }
}
impl ::core::default::Default for WHV_PROCESSOR_INTERCEPT_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_PROCESSOR_INTERCEPT_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.PageInvalidations == other.PageInvalidations && self.ControlRegisterAccesses == other.ControlRegisterAccesses && self.IoInstructions == other.IoInstructions && self.HaltInstructions == other.HaltInstructions && self.CpuidInstructions == other.CpuidInstructions && self.MsrAccesses == other.MsrAccesses && self.OtherIntercepts == other.OtherIntercepts && self.PendingInterrupts == other.PendingInterrupts && self.EmulatedInstructions == other.EmulatedInstructions && self.DebugRegisterAccesses == other.DebugRegisterAccesses && self.PageFaultIntercepts == other.PageFaultIntercepts && self.NestedPageFaultIntercepts == other.NestedPageFaultIntercepts && self.Hypercalls == other.Hypercalls && self.RdpmcInstructions == other.RdpmcInstructions
    }
}
impl ::core::cmp::Eq for WHV_PROCESSOR_INTERCEPT_COUNTERS {}
impl ::core::fmt::Debug for WHV_PROCESSOR_INTERCEPT_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_PROCESSOR_INTERCEPT_COUNTERS")
            .field("PageInvalidations", &self.PageInvalidations)
            .field("ControlRegisterAccesses", &self.ControlRegisterAccesses)
            .field("IoInstructions", &self.IoInstructions)
            .field("HaltInstructions", &self.HaltInstructions)
            .field("CpuidInstructions", &self.CpuidInstructions)
            .field("MsrAccesses", &self.MsrAccesses)
            .field("OtherIntercepts", &self.OtherIntercepts)
            .field("PendingInterrupts", &self.PendingInterrupts)
            .field("EmulatedInstructions", &self.EmulatedInstructions)
            .field("DebugRegisterAccesses", &self.DebugRegisterAccesses)
            .field("PageFaultIntercepts", &self.PageFaultIntercepts)
            .field("NestedPageFaultIntercepts", &self.NestedPageFaultIntercepts)
            .field("Hypercalls", &self.Hypercalls)
            .field("RdpmcInstructions", &self.RdpmcInstructions)
            .finish()
    }
}
impl ::core::default::Default for WHV_PROCESSOR_PERFMON_FEATURES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_PROCESSOR_RUNTIME_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_PROCESSOR_RUNTIME_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.TotalRuntime100ns == other.TotalRuntime100ns && self.HypervisorRuntime100ns == other.HypervisorRuntime100ns
    }
}
impl ::core::cmp::Eq for WHV_PROCESSOR_RUNTIME_COUNTERS {}
impl ::core::fmt::Debug for WHV_PROCESSOR_RUNTIME_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_PROCESSOR_RUNTIME_COUNTERS").field("TotalRuntime100ns", &self.TotalRuntime100ns).field("HypervisorRuntime100ns", &self.HypervisorRuntime100ns).finish()
    }
}
impl ::core::default::Default for WHV_PROCESSOR_SYNTHETIC_FEATURES_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_PROCESSOR_SYNTHETIC_FEATURES_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.SyntheticInterruptsCount == other.SyntheticInterruptsCount && self.LongSpinWaitHypercallsCount == other.LongSpinWaitHypercallsCount && self.OtherHypercallsCount == other.OtherHypercallsCount && self.SyntheticInterruptHypercallsCount == other.SyntheticInterruptHypercallsCount && self.VirtualInterruptHypercallsCount == other.VirtualInterruptHypercallsCount && self.VirtualMmuHypercallsCount == other.VirtualMmuHypercallsCount
    }
}
impl ::core::cmp::Eq for WHV_PROCESSOR_SYNTHETIC_FEATURES_COUNTERS {}
impl ::core::fmt::Debug for WHV_PROCESSOR_SYNTHETIC_FEATURES_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_PROCESSOR_SYNTHETIC_FEATURES_COUNTERS")
            .field("SyntheticInterruptsCount", &self.SyntheticInterruptsCount)
            .field("LongSpinWaitHypercallsCount", &self.LongSpinWaitHypercallsCount)
            .field("OtherHypercallsCount", &self.OtherHypercallsCount)
            .field("SyntheticInterruptHypercallsCount", &self.SyntheticInterruptHypercallsCount)
            .field("VirtualInterruptHypercallsCount", &self.VirtualInterruptHypercallsCount)
            .field("VirtualMmuHypercallsCount", &self.VirtualMmuHypercallsCount)
            .finish()
    }
}
impl ::core::default::Default for WHV_PROCESSOR_VENDOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_PROCESSOR_VENDOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_PROCESSOR_VENDOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_PROCESSOR_XSAVE_FEATURES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_REGISTER_NAME {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_REGISTER_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_REGISTER_NAME").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_REGISTER_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_RUN_VP_CANCELED_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_RUN_VP_CANCELED_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.CancelReason == other.CancelReason
    }
}
impl ::core::cmp::Eq for WHV_RUN_VP_CANCELED_CONTEXT {}
impl ::core::fmt::Debug for WHV_RUN_VP_CANCELED_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_RUN_VP_CANCELED_CONTEXT").field("CancelReason", &self.CancelReason).finish()
    }
}
impl ::core::default::Default for WHV_RUN_VP_CANCEL_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_RUN_VP_CANCEL_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_RUN_VP_CANCEL_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_RUN_VP_EXIT_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_RUN_VP_EXIT_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_RUN_VP_EXIT_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_RUN_VP_EXIT_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_SCHEDULER_FEATURES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WHV_SRIOV_RESOURCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WHV_SRIOV_RESOURCE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.PnpInstanceId == other.PnpInstanceId && self.VirtualFunctionId == other.VirtualFunctionId && self.VirtualFunctionIndex == other.VirtualFunctionIndex && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WHV_SRIOV_RESOURCE_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WHV_SRIOV_RESOURCE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_SRIOV_RESOURCE_DESCRIPTOR").field("PnpInstanceId", &self.PnpInstanceId).field("VirtualFunctionId", &self.VirtualFunctionId).field("VirtualFunctionIndex", &self.VirtualFunctionIndex).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for WHV_SYNIC_EVENT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_SYNIC_EVENT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.VpIndex == other.VpIndex && self.TargetSint == other.TargetSint && self.Reserved == other.Reserved && self.FlagNumber == other.FlagNumber
    }
}
impl ::core::cmp::Eq for WHV_SYNIC_EVENT_PARAMETERS {}
impl ::core::fmt::Debug for WHV_SYNIC_EVENT_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_SYNIC_EVENT_PARAMETERS").field("VpIndex", &self.VpIndex).field("TargetSint", &self.TargetSint).field("Reserved", &self.Reserved).field("FlagNumber", &self.FlagNumber).finish()
    }
}
impl ::core::default::Default for WHV_SYNIC_SINT_DELIVERABLE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_SYNIC_SINT_DELIVERABLE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.DeliverableSints == other.DeliverableSints && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for WHV_SYNIC_SINT_DELIVERABLE_CONTEXT {}
impl ::core::fmt::Debug for WHV_SYNIC_SINT_DELIVERABLE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_SYNIC_SINT_DELIVERABLE_CONTEXT").field("DeliverableSints", &self.DeliverableSints).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::default::Default for WHV_SYNTHETIC_PROCESSOR_FEATURES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_TRANSLATE_GVA_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_TRANSLATE_GVA_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_TRANSLATE_GVA_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WHV_TRANSLATE_GVA_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WHV_TRANSLATE_GVA_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WHV_TRANSLATE_GVA_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WHV_TRANSLATE_GVA_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WHV_TRANSLATE_GVA_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for WHV_TRANSLATE_GVA_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_TRANSLATE_GVA_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.ResultCode == other.ResultCode && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for WHV_TRANSLATE_GVA_RESULT {}
impl ::core::fmt::Debug for WHV_TRANSLATE_GVA_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_TRANSLATE_GVA_RESULT").field("ResultCode", &self.ResultCode).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for WHV_TRANSLATE_GVA_RESULT_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_TRANSLATE_GVA_RESULT_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_TRANSLATE_GVA_RESULT_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_TRIGGER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_TRIGGER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_TRIGGER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_TRIGGER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_UINT128 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_VIRTUAL_PROCESSOR_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_VIRTUAL_PROCESSOR_PROPERTY_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_VIRTUAL_PROCESSOR_PROPERTY_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_VIRTUAL_PROCESSOR_PROPERTY_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_VIRTUAL_PROCESSOR_STATE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_VIRTUAL_PROCESSOR_STATE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_VIRTUAL_PROCESSOR_STATE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_VPCI_DEVICE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_VPCI_DEVICE_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_VPCI_DEVICE_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_VPCI_DEVICE_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_VPCI_DEVICE_PROPERTY_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_VPCI_DEVICE_PROPERTY_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_VPCI_DEVICE_PROPERTY_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_VPCI_DEVICE_REGISTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_VPCI_DEVICE_REGISTER {
    fn eq(&self, other: &Self) -> bool {
        self.Location == other.Location && self.SizeInBytes == other.SizeInBytes && self.OffsetInBytes == other.OffsetInBytes
    }
}
impl ::core::cmp::Eq for WHV_VPCI_DEVICE_REGISTER {}
impl ::core::fmt::Debug for WHV_VPCI_DEVICE_REGISTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_VPCI_DEVICE_REGISTER").field("Location", &self.Location).field("SizeInBytes", &self.SizeInBytes).field("OffsetInBytes", &self.OffsetInBytes).finish()
    }
}
impl ::core::default::Default for WHV_VPCI_DEVICE_REGISTER_SPACE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_VPCI_DEVICE_REGISTER_SPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_VPCI_DEVICE_REGISTER_SPACE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_VPCI_HARDWARE_IDS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_VPCI_HARDWARE_IDS {
    fn eq(&self, other: &Self) -> bool {
        self.VendorID == other.VendorID && self.DeviceID == other.DeviceID && self.RevisionID == other.RevisionID && self.ProgIf == other.ProgIf && self.SubClass == other.SubClass && self.BaseClass == other.BaseClass && self.SubVendorID == other.SubVendorID && self.SubSystemID == other.SubSystemID
    }
}
impl ::core::cmp::Eq for WHV_VPCI_HARDWARE_IDS {}
impl ::core::fmt::Debug for WHV_VPCI_HARDWARE_IDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_VPCI_HARDWARE_IDS").field("VendorID", &self.VendorID).field("DeviceID", &self.DeviceID).field("RevisionID", &self.RevisionID).field("ProgIf", &self.ProgIf).field("SubClass", &self.SubClass).field("BaseClass", &self.BaseClass).field("SubVendorID", &self.SubVendorID).field("SubSystemID", &self.SubSystemID).finish()
    }
}
impl ::core::default::Default for WHV_VPCI_INTERRUPT_TARGET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_VPCI_INTERRUPT_TARGET {
    fn eq(&self, other: &Self) -> bool {
        self.Vector == other.Vector && self.Flags == other.Flags && self.ProcessorCount == other.ProcessorCount && self.Processors == other.Processors
    }
}
impl ::core::cmp::Eq for WHV_VPCI_INTERRUPT_TARGET {}
impl ::core::fmt::Debug for WHV_VPCI_INTERRUPT_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_VPCI_INTERRUPT_TARGET").field("Vector", &self.Vector).field("Flags", &self.Flags).field("ProcessorCount", &self.ProcessorCount).field("Processors", &self.Processors).finish()
    }
}
impl ::core::default::Default for WHV_VPCI_INTERRUPT_TARGET_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_VPCI_INTERRUPT_TARGET_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_VPCI_INTERRUPT_TARGET_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WHV_VPCI_INTERRUPT_TARGET_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WHV_VPCI_INTERRUPT_TARGET_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WHV_VPCI_INTERRUPT_TARGET_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WHV_VPCI_INTERRUPT_TARGET_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WHV_VPCI_INTERRUPT_TARGET_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for WHV_VPCI_MMIO_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_VPCI_MMIO_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.Location == other.Location && self.Flags == other.Flags && self.SizeInBytes == other.SizeInBytes && self.OffsetInBytes == other.OffsetInBytes && self.VirtualAddress == other.VirtualAddress
    }
}
impl ::core::cmp::Eq for WHV_VPCI_MMIO_MAPPING {}
impl ::core::fmt::Debug for WHV_VPCI_MMIO_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_VPCI_MMIO_MAPPING").field("Location", &self.Location).field("Flags", &self.Flags).field("SizeInBytes", &self.SizeInBytes).field("OffsetInBytes", &self.OffsetInBytes).field("VirtualAddress", &self.VirtualAddress).finish()
    }
}
impl ::core::default::Default for WHV_VPCI_MMIO_RANGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_VPCI_MMIO_RANGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_VPCI_MMIO_RANGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WHV_VPCI_MMIO_RANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WHV_VPCI_MMIO_RANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WHV_VPCI_MMIO_RANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WHV_VPCI_MMIO_RANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WHV_VPCI_MMIO_RANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for WHV_VPCI_PROBED_BARS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_VPCI_PROBED_BARS {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::core::cmp::Eq for WHV_VPCI_PROBED_BARS {}
impl ::core::fmt::Debug for WHV_VPCI_PROBED_BARS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_VPCI_PROBED_BARS").field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for WHV_VP_EXCEPTION_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_VP_EXCEPTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_VP_EXIT_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_X64_APIC_EOI_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_X64_APIC_EOI_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.InterruptVector == other.InterruptVector
    }
}
impl ::core::cmp::Eq for WHV_X64_APIC_EOI_CONTEXT {}
impl ::core::fmt::Debug for WHV_X64_APIC_EOI_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_X64_APIC_EOI_CONTEXT").field("InterruptVector", &self.InterruptVector).finish()
    }
}
impl ::core::default::Default for WHV_X64_APIC_INIT_SIPI_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_X64_APIC_INIT_SIPI_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ApicIcr == other.ApicIcr
    }
}
impl ::core::cmp::Eq for WHV_X64_APIC_INIT_SIPI_CONTEXT {}
impl ::core::fmt::Debug for WHV_X64_APIC_INIT_SIPI_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_X64_APIC_INIT_SIPI_CONTEXT").field("ApicIcr", &self.ApicIcr).finish()
    }
}
impl ::core::default::Default for WHV_X64_APIC_SMI_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_X64_APIC_SMI_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ApicIcr == other.ApicIcr
    }
}
impl ::core::cmp::Eq for WHV_X64_APIC_SMI_CONTEXT {}
impl ::core::fmt::Debug for WHV_X64_APIC_SMI_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_X64_APIC_SMI_CONTEXT").field("ApicIcr", &self.ApicIcr).finish()
    }
}
impl ::core::default::Default for WHV_X64_APIC_WRITE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_X64_APIC_WRITE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Reserved == other.Reserved && self.WriteValue == other.WriteValue
    }
}
impl ::core::cmp::Eq for WHV_X64_APIC_WRITE_CONTEXT {}
impl ::core::fmt::Debug for WHV_X64_APIC_WRITE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_X64_APIC_WRITE_CONTEXT").field("Type", &self.Type).field("Reserved", &self.Reserved).field("WriteValue", &self.WriteValue).finish()
    }
}
impl ::core::default::Default for WHV_X64_APIC_WRITE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_X64_APIC_WRITE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_X64_APIC_WRITE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_X64_CPUID_ACCESS_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_X64_CPUID_ACCESS_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Rax == other.Rax && self.Rcx == other.Rcx && self.Rdx == other.Rdx && self.Rbx == other.Rbx && self.DefaultResultRax == other.DefaultResultRax && self.DefaultResultRcx == other.DefaultResultRcx && self.DefaultResultRdx == other.DefaultResultRdx && self.DefaultResultRbx == other.DefaultResultRbx
    }
}
impl ::core::cmp::Eq for WHV_X64_CPUID_ACCESS_CONTEXT {}
impl ::core::fmt::Debug for WHV_X64_CPUID_ACCESS_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_X64_CPUID_ACCESS_CONTEXT").field("Rax", &self.Rax).field("Rcx", &self.Rcx).field("Rdx", &self.Rdx).field("Rbx", &self.Rbx).field("DefaultResultRax", &self.DefaultResultRax).field("DefaultResultRcx", &self.DefaultResultRcx).field("DefaultResultRdx", &self.DefaultResultRdx).field("DefaultResultRbx", &self.DefaultResultRbx).finish()
    }
}
impl ::core::default::Default for WHV_X64_CPUID_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_X64_CPUID_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.Function == other.Function && self.Reserved == other.Reserved && self.Eax == other.Eax && self.Ebx == other.Ebx && self.Ecx == other.Ecx && self.Edx == other.Edx
    }
}
impl ::core::cmp::Eq for WHV_X64_CPUID_RESULT {}
impl ::core::fmt::Debug for WHV_X64_CPUID_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_X64_CPUID_RESULT").field("Function", &self.Function).field("Reserved", &self.Reserved).field("Eax", &self.Eax).field("Ebx", &self.Ebx).field("Ecx", &self.Ecx).field("Edx", &self.Edx).finish()
    }
}
impl ::core::default::Default for WHV_X64_CPUID_RESULT2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_X64_CPUID_RESULT2 {
    fn eq(&self, other: &Self) -> bool {
        self.Function == other.Function && self.Index == other.Index && self.VpIndex == other.VpIndex && self.Flags == other.Flags && self.Output == other.Output && self.Mask == other.Mask
    }
}
impl ::core::cmp::Eq for WHV_X64_CPUID_RESULT2 {}
impl ::core::fmt::Debug for WHV_X64_CPUID_RESULT2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_X64_CPUID_RESULT2").field("Function", &self.Function).field("Index", &self.Index).field("VpIndex", &self.VpIndex).field("Flags", &self.Flags).field("Output", &self.Output).field("Mask", &self.Mask).finish()
    }
}
impl ::core::default::Default for WHV_X64_CPUID_RESULT2_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_X64_CPUID_RESULT2_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_X64_CPUID_RESULT2_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WHV_X64_CPUID_RESULT2_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WHV_X64_CPUID_RESULT2_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WHV_X64_CPUID_RESULT2_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WHV_X64_CPUID_RESULT2_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WHV_X64_CPUID_RESULT2_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_X64_FP_CONTROL_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_X64_FP_REGISTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.DeliverableType == other.DeliverableType
    }
}
impl ::core::cmp::Eq for WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT {}
impl ::core::fmt::Debug for WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT").field("DeliverableType", &self.DeliverableType).finish()
    }
}
impl ::core::default::Default for WHV_X64_INTERRUPT_STATE_REGISTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_X64_IO_PORT_ACCESS_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_X64_IO_PORT_ACCESS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_X64_LOCAL_APIC_EMULATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_X64_LOCAL_APIC_EMULATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_X64_LOCAL_APIC_EMULATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_X64_MSR_ACCESS_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_X64_MSR_ACCESS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_X64_MSR_EXIT_BITMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_X64_PENDING_DEBUG_EXCEPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_X64_PENDING_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_X64_PENDING_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_X64_PENDING_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_X64_PENDING_EXCEPTION_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_X64_PENDING_EXT_INT_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_X64_PENDING_INTERRUPTION_REGISTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_X64_PENDING_INTERRUPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_X64_PENDING_INTERRUPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_X64_PENDING_INTERRUPTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_X64_RDTSC_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_X64_RDTSC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_X64_SEGMENT_REGISTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_X64_TABLE_REGISTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_X64_TABLE_REGISTER {
    fn eq(&self, other: &Self) -> bool {
        self.Pad == other.Pad && self.Limit == other.Limit && self.Base == other.Base
    }
}
impl ::core::cmp::Eq for WHV_X64_TABLE_REGISTER {}
impl ::core::fmt::Debug for WHV_X64_TABLE_REGISTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_X64_TABLE_REGISTER").field("Pad", &self.Pad).field("Limit", &self.Limit).field("Base", &self.Base).finish()
    }
}
impl ::core::default::Default for WHV_X64_UNSUPPORTED_FEATURE_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WHV_X64_UNSUPPORTED_FEATURE_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WHV_X64_UNSUPPORTED_FEATURE_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WHV_X64_UNSUPPORTED_FEATURE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WHV_X64_UNSUPPORTED_FEATURE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.FeatureCode == other.FeatureCode && self.Reserved == other.Reserved && self.FeatureParameter == other.FeatureParameter
    }
}
impl ::core::cmp::Eq for WHV_X64_UNSUPPORTED_FEATURE_CONTEXT {}
impl ::core::fmt::Debug for WHV_X64_UNSUPPORTED_FEATURE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WHV_X64_UNSUPPORTED_FEATURE_CONTEXT").field("FeatureCode", &self.FeatureCode).field("Reserved", &self.Reserved).field("FeatureParameter", &self.FeatureParameter).finish()
    }
}
impl ::core::default::Default for WHV_X64_VP_EXECUTION_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for WHV_X64_XMM_CONTROL_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
