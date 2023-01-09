impl ::core::default::Default for ASYNC_DUPLICATE_EXTENTS_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ASYNC_DUPLICATE_EXTENTS_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.State == other.State && self.SourceFileOffset == other.SourceFileOffset && self.TargetFileOffset == other.TargetFileOffset && self.ByteCount == other.ByteCount && self.BytesDuplicated == other.BytesDuplicated
    }
}
impl ::core::cmp::Eq for ASYNC_DUPLICATE_EXTENTS_STATUS {}
impl ::core::fmt::Debug for ASYNC_DUPLICATE_EXTENTS_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ASYNC_DUPLICATE_EXTENTS_STATUS").field("Version", &self.Version).field("State", &self.State).field("SourceFileOffset", &self.SourceFileOffset).field("TargetFileOffset", &self.TargetFileOffset).field("ByteCount", &self.ByteCount).field("BytesDuplicated", &self.BytesDuplicated).finish()
    }
}
impl ::core::default::Default for BIN_COUNT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BIN_COUNT {
    fn eq(&self, other: &Self) -> bool {
        self.BinRange == other.BinRange && self.BinCount == other.BinCount
    }
}
impl ::core::cmp::Eq for BIN_COUNT {}
impl ::core::fmt::Debug for BIN_COUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIN_COUNT").field("BinRange", &self.BinRange).field("BinCount", &self.BinCount).finish()
    }
}
impl ::core::default::Default for BIN_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BIN_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartValue == other.StartValue && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for BIN_RANGE {}
impl ::core::fmt::Debug for BIN_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIN_RANGE").field("StartValue", &self.StartValue).field("Length", &self.Length).finish()
    }
}
impl ::core::default::Default for BIN_RESULTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BIN_RESULTS {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfBins == other.NumberOfBins && self.BinCounts == other.BinCounts
    }
}
impl ::core::cmp::Eq for BIN_RESULTS {}
impl ::core::fmt::Debug for BIN_RESULTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BIN_RESULTS").field("NumberOfBins", &self.NumberOfBins).field("BinCounts", &self.BinCounts).finish()
    }
}
impl ::core::default::Default for BIN_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for BIN_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BIN_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for BOOT_AREA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BOOT_AREA_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.BootSectorCount == other.BootSectorCount && self.BootSectors == other.BootSectors
    }
}
impl ::core::cmp::Eq for BOOT_AREA_INFO {}
impl ::core::fmt::Debug for BOOT_AREA_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BOOT_AREA_INFO").field("BootSectorCount", &self.BootSectorCount).field("BootSectors", &self.BootSectors).finish()
    }
}
impl ::core::default::Default for BOOT_AREA_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BOOT_AREA_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for BOOT_AREA_INFO_0 {}
impl ::core::fmt::Debug for BOOT_AREA_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BOOT_AREA_INFO_0").field("Offset", &self.Offset).finish()
    }
}
impl ::core::default::Default for BULK_SECURITY_TEST_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for BULK_SECURITY_TEST_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DesiredAccess == other.DesiredAccess && self.SecurityIds == other.SecurityIds
    }
}
impl ::core::cmp::Eq for BULK_SECURITY_TEST_DATA {}
impl ::core::fmt::Debug for BULK_SECURITY_TEST_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BULK_SECURITY_TEST_DATA").field("DesiredAccess", &self.DesiredAccess).field("SecurityIds", &self.SecurityIds).finish()
    }
}
impl ::core::default::Default for CHANGER_DEVICE_PROBLEM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CHANGER_DEVICE_PROBLEM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANGER_DEVICE_PROBLEM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for CHANGER_ELEMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CHANGER_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.ElementType == other.ElementType && self.ElementAddress == other.ElementAddress
    }
}
impl ::core::cmp::Eq for CHANGER_ELEMENT {}
impl ::core::fmt::Debug for CHANGER_ELEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_ELEMENT").field("ElementType", &self.ElementType).field("ElementAddress", &self.ElementAddress).finish()
    }
}
impl ::core::default::Default for CHANGER_ELEMENT_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CHANGER_ELEMENT_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Element == other.Element && self.NumberOfElements == other.NumberOfElements
    }
}
impl ::core::cmp::Eq for CHANGER_ELEMENT_LIST {}
impl ::core::fmt::Debug for CHANGER_ELEMENT_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_ELEMENT_LIST").field("Element", &self.Element).field("NumberOfElements", &self.NumberOfElements).finish()
    }
}
impl ::core::default::Default for CHANGER_ELEMENT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CHANGER_ELEMENT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Element == other.Element && self.SrcElementAddress == other.SrcElementAddress && self.Flags == other.Flags && self.ExceptionCode == other.ExceptionCode && self.TargetId == other.TargetId && self.Lun == other.Lun && self.Reserved == other.Reserved && self.PrimaryVolumeID == other.PrimaryVolumeID && self.AlternateVolumeID == other.AlternateVolumeID
    }
}
impl ::core::cmp::Eq for CHANGER_ELEMENT_STATUS {}
impl ::core::fmt::Debug for CHANGER_ELEMENT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_ELEMENT_STATUS").field("Element", &self.Element).field("SrcElementAddress", &self.SrcElementAddress).field("Flags", &self.Flags).field("ExceptionCode", &self.ExceptionCode).field("TargetId", &self.TargetId).field("Lun", &self.Lun).field("Reserved", &self.Reserved).field("PrimaryVolumeID", &self.PrimaryVolumeID).field("AlternateVolumeID", &self.AlternateVolumeID).finish()
    }
}
impl ::core::default::Default for CHANGER_ELEMENT_STATUS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CHANGER_ELEMENT_STATUS_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Element == other.Element && self.SrcElementAddress == other.SrcElementAddress && self.Flags == other.Flags && self.ExceptionCode == other.ExceptionCode && self.TargetId == other.TargetId && self.Lun == other.Lun && self.Reserved == other.Reserved && self.PrimaryVolumeID == other.PrimaryVolumeID && self.AlternateVolumeID == other.AlternateVolumeID && self.VendorIdentification == other.VendorIdentification && self.ProductIdentification == other.ProductIdentification && self.SerialNumber == other.SerialNumber
    }
}
impl ::core::cmp::Eq for CHANGER_ELEMENT_STATUS_EX {}
impl ::core::fmt::Debug for CHANGER_ELEMENT_STATUS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_ELEMENT_STATUS_EX")
            .field("Element", &self.Element)
            .field("SrcElementAddress", &self.SrcElementAddress)
            .field("Flags", &self.Flags)
            .field("ExceptionCode", &self.ExceptionCode)
            .field("TargetId", &self.TargetId)
            .field("Lun", &self.Lun)
            .field("Reserved", &self.Reserved)
            .field("PrimaryVolumeID", &self.PrimaryVolumeID)
            .field("AlternateVolumeID", &self.AlternateVolumeID)
            .field("VendorIdentification", &self.VendorIdentification)
            .field("ProductIdentification", &self.ProductIdentification)
            .field("SerialNumber", &self.SerialNumber)
            .finish()
    }
}
impl ::core::default::Default for CHANGER_ELEMENT_STATUS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CHANGER_ELEMENT_STATUS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANGER_ELEMENT_STATUS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CHANGER_ELEMENT_STATUS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CHANGER_ELEMENT_STATUS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CHANGER_ELEMENT_STATUS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CHANGER_ELEMENT_STATUS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CHANGER_ELEMENT_STATUS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANGER_EXCHANGE_MEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHANGER_EXCHANGE_MEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.Transport == other.Transport && self.Source == other.Source && self.Destination1 == other.Destination1 && self.Destination2 == other.Destination2 && self.Flip1 == other.Flip1 && self.Flip2 == other.Flip2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHANGER_EXCHANGE_MEDIUM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CHANGER_EXCHANGE_MEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_EXCHANGE_MEDIUM").field("Transport", &self.Transport).field("Source", &self.Source).field("Destination1", &self.Destination1).field("Destination2", &self.Destination2).field("Flip1", &self.Flip1).field("Flip2", &self.Flip2).finish()
    }
}
impl ::core::default::Default for CHANGER_FEATURES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CHANGER_FEATURES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHANGER_FEATURES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CHANGER_FEATURES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CHANGER_FEATURES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CHANGER_FEATURES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CHANGER_FEATURES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CHANGER_FEATURES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANGER_INITIALIZE_ELEMENT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHANGER_INITIALIZE_ELEMENT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.ElementList == other.ElementList && self.BarCodeScan == other.BarCodeScan
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHANGER_INITIALIZE_ELEMENT_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CHANGER_INITIALIZE_ELEMENT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_INITIALIZE_ELEMENT_STATUS").field("ElementList", &self.ElementList).field("BarCodeScan", &self.BarCodeScan).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANGER_MOVE_MEDIUM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHANGER_MOVE_MEDIUM {
    fn eq(&self, other: &Self) -> bool {
        self.Transport == other.Transport && self.Source == other.Source && self.Destination == other.Destination && self.Flip == other.Flip
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHANGER_MOVE_MEDIUM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CHANGER_MOVE_MEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_MOVE_MEDIUM").field("Transport", &self.Transport).field("Source", &self.Source).field("Destination", &self.Destination).field("Flip", &self.Flip).finish()
    }
}
impl ::core::default::Default for CHANGER_PRODUCT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CHANGER_PRODUCT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.VendorId == other.VendorId && self.ProductId == other.ProductId && self.Revision == other.Revision && self.SerialNumber == other.SerialNumber && self.DeviceType == other.DeviceType
    }
}
impl ::core::cmp::Eq for CHANGER_PRODUCT_DATA {}
impl ::core::fmt::Debug for CHANGER_PRODUCT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_PRODUCT_DATA").field("VendorId", &self.VendorId).field("ProductId", &self.ProductId).field("Revision", &self.Revision).field("SerialNumber", &self.SerialNumber).field("DeviceType", &self.DeviceType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANGER_READ_ELEMENT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHANGER_READ_ELEMENT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.ElementList == other.ElementList && self.VolumeTagInfo == other.VolumeTagInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHANGER_READ_ELEMENT_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CHANGER_READ_ELEMENT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_READ_ELEMENT_STATUS").field("ElementList", &self.ElementList).field("VolumeTagInfo", &self.VolumeTagInfo).finish()
    }
}
impl ::core::default::Default for CHANGER_SEND_VOLUME_TAG_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CHANGER_SEND_VOLUME_TAG_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.StartingElement == other.StartingElement && self.ActionCode == other.ActionCode && self.VolumeIDTemplate == other.VolumeIDTemplate
    }
}
impl ::core::cmp::Eq for CHANGER_SEND_VOLUME_TAG_INFORMATION {}
impl ::core::fmt::Debug for CHANGER_SEND_VOLUME_TAG_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_SEND_VOLUME_TAG_INFORMATION").field("StartingElement", &self.StartingElement).field("ActionCode", &self.ActionCode).field("VolumeIDTemplate", &self.VolumeIDTemplate).finish()
    }
}
impl ::core::default::Default for CHANGER_SET_ACCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CHANGER_SET_ACCESS {
    fn eq(&self, other: &Self) -> bool {
        self.Element == other.Element && self.Control == other.Control
    }
}
impl ::core::cmp::Eq for CHANGER_SET_ACCESS {}
impl ::core::fmt::Debug for CHANGER_SET_ACCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_SET_ACCESS").field("Element", &self.Element).field("Control", &self.Control).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHANGER_SET_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CHANGER_SET_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Transport == other.Transport && self.Destination == other.Destination && self.Flip == other.Flip
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CHANGER_SET_POSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CHANGER_SET_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CHANGER_SET_POSITION").field("Transport", &self.Transport).field("Destination", &self.Destination).field("Flip", &self.Flip).finish()
    }
}
impl ::core::default::Default for CLASS_MEDIA_CHANGE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLASS_MEDIA_CHANGE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.MediaChangeCount == other.MediaChangeCount && self.NewState == other.NewState
    }
}
impl ::core::cmp::Eq for CLASS_MEDIA_CHANGE_CONTEXT {}
impl ::core::fmt::Debug for CLASS_MEDIA_CHANGE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLASS_MEDIA_CHANGE_CONTEXT").field("MediaChangeCount", &self.MediaChangeCount).field("NewState", &self.NewState).finish()
    }
}
impl ::core::default::Default for CLUSTER_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CLUSTER_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartingCluster == other.StartingCluster && self.ClusterCount == other.ClusterCount
    }
}
impl ::core::cmp::Eq for CLUSTER_RANGE {}
impl ::core::fmt::Debug for CLUSTER_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CLUSTER_RANGE").field("StartingCluster", &self.StartingCluster).field("ClusterCount", &self.ClusterCount).finish()
    }
}
impl ::core::default::Default for CONTAINER_ROOT_INFO_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONTAINER_ROOT_INFO_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for CONTAINER_ROOT_INFO_INPUT {}
impl ::core::fmt::Debug for CONTAINER_ROOT_INFO_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONTAINER_ROOT_INFO_INPUT").field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for CONTAINER_ROOT_INFO_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONTAINER_ROOT_INFO_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ContainerRootIdLength == other.ContainerRootIdLength && self.ContainerRootId == other.ContainerRootId
    }
}
impl ::core::cmp::Eq for CONTAINER_ROOT_INFO_OUTPUT {}
impl ::core::fmt::Debug for CONTAINER_ROOT_INFO_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONTAINER_ROOT_INFO_OUTPUT").field("ContainerRootIdLength", &self.ContainerRootIdLength).field("ContainerRootId", &self.ContainerRootId).finish()
    }
}
impl ::core::default::Default for CONTAINER_VOLUME_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CONTAINER_VOLUME_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for CONTAINER_VOLUME_STATE {}
impl ::core::fmt::Debug for CONTAINER_VOLUME_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONTAINER_VOLUME_STATE").field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for CREATE_DISK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for CREATE_DISK_GPT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CREATE_DISK_GPT {
    fn eq(&self, other: &Self) -> bool {
        self.DiskId == other.DiskId && self.MaxPartitionCount == other.MaxPartitionCount
    }
}
impl ::core::cmp::Eq for CREATE_DISK_GPT {}
impl ::core::fmt::Debug for CREATE_DISK_GPT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATE_DISK_GPT").field("DiskId", &self.DiskId).field("MaxPartitionCount", &self.MaxPartitionCount).finish()
    }
}
impl ::core::default::Default for CREATE_DISK_MBR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CREATE_DISK_MBR {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature
    }
}
impl ::core::cmp::Eq for CREATE_DISK_MBR {}
impl ::core::fmt::Debug for CREATE_DISK_MBR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATE_DISK_MBR").field("Signature", &self.Signature).finish()
    }
}
impl ::core::default::Default for CREATE_USN_JOURNAL_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CREATE_USN_JOURNAL_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumSize == other.MaximumSize && self.AllocationDelta == other.AllocationDelta
    }
}
impl ::core::cmp::Eq for CREATE_USN_JOURNAL_DATA {}
impl ::core::fmt::Debug for CREATE_USN_JOURNAL_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CREATE_USN_JOURNAL_DATA").field("MaximumSize", &self.MaximumSize).field("AllocationDelta", &self.AllocationDelta).finish()
    }
}
impl ::core::default::Default for CSVFS_DISK_CONNECTIVITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CSVFS_DISK_CONNECTIVITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSVFS_DISK_CONNECTIVITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for CSV_CONTROL_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CSV_CONTROL_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CSV_CONTROL_OP").field(&self.0).finish()
    }
}
impl ::core::default::Default for CSV_CONTROL_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CSV_CONTROL_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.Operation == other.Operation && self.Unused == other.Unused
    }
}
impl ::core::cmp::Eq for CSV_CONTROL_PARAM {}
impl ::core::fmt::Debug for CSV_CONTROL_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_CONTROL_PARAM").field("Operation", &self.Operation).field("Unused", &self.Unused).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CSV_IS_OWNED_BY_CSVFS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CSV_IS_OWNED_BY_CSVFS {
    fn eq(&self, other: &Self) -> bool {
        self.OwnedByCSVFS == other.OwnedByCSVFS
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CSV_IS_OWNED_BY_CSVFS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CSV_IS_OWNED_BY_CSVFS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_IS_OWNED_BY_CSVFS").field("OwnedByCSVFS", &self.OwnedByCSVFS).finish()
    }
}
impl ::core::default::Default for CSV_MGMT_LOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CSV_MGMT_LOCK {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for CSV_MGMT_LOCK {}
impl ::core::fmt::Debug for CSV_MGMT_LOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_MGMT_LOCK").field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for CSV_NAMESPACE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CSV_NAMESPACE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.DeviceNumber == other.DeviceNumber && self.StartingOffset == other.StartingOffset && self.SectorSize == other.SectorSize
    }
}
impl ::core::cmp::Eq for CSV_NAMESPACE_INFO {}
impl ::core::fmt::Debug for CSV_NAMESPACE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_NAMESPACE_INFO").field("Version", &self.Version).field("DeviceNumber", &self.DeviceNumber).field("StartingOffset", &self.StartingOffset).field("SectorSize", &self.SectorSize).finish()
    }
}
impl ::core::default::Default for CSV_QUERY_FILE_REVISION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CSV_QUERY_FILE_REVISION {
    fn eq(&self, other: &Self) -> bool {
        self.FileId == other.FileId && self.FileRevision == other.FileRevision
    }
}
impl ::core::cmp::Eq for CSV_QUERY_FILE_REVISION {}
impl ::core::fmt::Debug for CSV_QUERY_FILE_REVISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_QUERY_FILE_REVISION").field("FileId", &self.FileId).field("FileRevision", &self.FileRevision).finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for CSV_QUERY_FILE_REVISION_FILE_ID_128 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for CSV_QUERY_FILE_REVISION_FILE_ID_128 {
    fn eq(&self, other: &Self) -> bool {
        self.FileId == other.FileId && self.FileRevision == other.FileRevision
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for CSV_QUERY_FILE_REVISION_FILE_ID_128 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::fmt::Debug for CSV_QUERY_FILE_REVISION_FILE_ID_128 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_QUERY_FILE_REVISION_FILE_ID_128").field("FileId", &self.FileId).field("FileRevision", &self.FileRevision).finish()
    }
}
impl ::core::default::Default for CSV_QUERY_MDS_PATH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CSV_QUERY_MDS_PATH {
    fn eq(&self, other: &Self) -> bool {
        self.MdsNodeId == other.MdsNodeId && self.DsNodeId == other.DsNodeId && self.PathLength == other.PathLength && self.Path == other.Path
    }
}
impl ::core::cmp::Eq for CSV_QUERY_MDS_PATH {}
impl ::core::fmt::Debug for CSV_QUERY_MDS_PATH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_QUERY_MDS_PATH").field("MdsNodeId", &self.MdsNodeId).field("DsNodeId", &self.DsNodeId).field("PathLength", &self.PathLength).field("Path", &self.Path).finish()
    }
}
impl ::core::default::Default for CSV_QUERY_MDS_PATH_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CSV_QUERY_MDS_PATH_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.RequiredSize == other.RequiredSize && self.MdsNodeId == other.MdsNodeId && self.DsNodeId == other.DsNodeId && self.Flags == other.Flags && self.DiskConnectivity == other.DiskConnectivity && self.VolumeId == other.VolumeId && self.IpAddressOffset == other.IpAddressOffset && self.IpAddressLength == other.IpAddressLength && self.PathOffset == other.PathOffset && self.PathLength == other.PathLength
    }
}
impl ::core::cmp::Eq for CSV_QUERY_MDS_PATH_V2 {}
impl ::core::fmt::Debug for CSV_QUERY_MDS_PATH_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_QUERY_MDS_PATH_V2").field("Version", &self.Version).field("RequiredSize", &self.RequiredSize).field("MdsNodeId", &self.MdsNodeId).field("DsNodeId", &self.DsNodeId).field("Flags", &self.Flags).field("DiskConnectivity", &self.DiskConnectivity).field("VolumeId", &self.VolumeId).field("IpAddressOffset", &self.IpAddressOffset).field("IpAddressLength", &self.IpAddressLength).field("PathOffset", &self.PathOffset).field("PathLength", &self.PathLength).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CSV_QUERY_REDIRECT_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CSV_QUERY_REDIRECT_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.MdsNodeId == other.MdsNodeId && self.DsNodeId == other.DsNodeId && self.FileRedirected == other.FileRedirected
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CSV_QUERY_REDIRECT_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CSV_QUERY_REDIRECT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_QUERY_REDIRECT_STATE").field("MdsNodeId", &self.MdsNodeId).field("DsNodeId", &self.DsNodeId).field("FileRedirected", &self.FileRedirected).finish()
    }
}
impl ::core::default::Default for CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.VetoedFromAltitudeIntegral == other.VetoedFromAltitudeIntegral && self.VetoedFromAltitudeDecimal == other.VetoedFromAltitudeDecimal && self.Reason == other.Reason
    }
}
impl ::core::cmp::Eq for CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {}
impl ::core::fmt::Debug for CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_QUERY_VETO_FILE_DIRECT_IO_OUTPUT").field("VetoedFromAltitudeIntegral", &self.VetoedFromAltitudeIntegral).field("VetoedFromAltitudeDecimal", &self.VetoedFromAltitudeDecimal).field("Reason", &self.Reason).finish()
    }
}
impl ::core::default::Default for CSV_QUERY_VOLUME_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CSV_QUERY_VOLUME_ID {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeId == other.VolumeId
    }
}
impl ::core::cmp::Eq for CSV_QUERY_VOLUME_ID {}
impl ::core::fmt::Debug for CSV_QUERY_VOLUME_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_QUERY_VOLUME_ID").field("VolumeId", &self.VolumeId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CSV_QUERY_VOLUME_REDIRECT_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CSV_QUERY_VOLUME_REDIRECT_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.MdsNodeId == other.MdsNodeId && self.DsNodeId == other.DsNodeId && self.IsDiskConnected == other.IsDiskConnected && self.ClusterEnableDirectIo == other.ClusterEnableDirectIo && self.DiskConnectivity == other.DiskConnectivity
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CSV_QUERY_VOLUME_REDIRECT_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CSV_QUERY_VOLUME_REDIRECT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_QUERY_VOLUME_REDIRECT_STATE").field("MdsNodeId", &self.MdsNodeId).field("DsNodeId", &self.DsNodeId).field("IsDiskConnected", &self.IsDiskConnected).field("ClusterEnableDirectIo", &self.ClusterEnableDirectIo).field("DiskConnectivity", &self.DiskConnectivity).finish()
    }
}
impl ::core::default::Default for CSV_SET_VOLUME_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for CSV_SET_VOLUME_ID {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeId == other.VolumeId
    }
}
impl ::core::cmp::Eq for CSV_SET_VOLUME_ID {}
impl ::core::fmt::Debug for CSV_SET_VOLUME_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CSV_SET_VOLUME_ID").field("VolumeId", &self.VolumeId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DECRYPTION_STATUS_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DECRYPTION_STATUS_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.NoEncryptedStreams == other.NoEncryptedStreams
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DECRYPTION_STATUS_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DECRYPTION_STATUS_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DECRYPTION_STATUS_BUFFER").field("NoEncryptedStreams", &self.NoEncryptedStreams).finish()
    }
}
impl ::core::default::Default for DELETE_USN_JOURNAL_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DELETE_USN_JOURNAL_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.UsnJournalID == other.UsnJournalID && self.DeleteFlags == other.DeleteFlags
    }
}
impl ::core::cmp::Eq for DELETE_USN_JOURNAL_DATA {}
impl ::core::fmt::Debug for DELETE_USN_JOURNAL_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DELETE_USN_JOURNAL_DATA").field("UsnJournalID", &self.UsnJournalID).field("DeleteFlags", &self.DeleteFlags).finish()
    }
}
impl ::core::default::Default for DETECTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DETECTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DETECTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEVICEDUMP_COLLECTION_TYPEIDE_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVICEDUMP_COLLECTION_TYPEIDE_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICEDUMP_COLLECTION_TYPEIDE_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEVICEDUMP_PRIVATE_SUBSECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICEDUMP_PUBLIC_SUBSECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DEVICEDUMP_RESTRICTED_SUBSECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICEDUMP_RESTRICTED_SUBSECTION {
    fn eq(&self, other: &Self) -> bool {
        self.bData == other.bData
    }
}
impl ::core::cmp::Eq for DEVICEDUMP_RESTRICTED_SUBSECTION {}
impl ::core::fmt::Debug for DEVICEDUMP_RESTRICTED_SUBSECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICEDUMP_RESTRICTED_SUBSECTION").field("bData", &self.bData).finish()
    }
}
impl ::core::default::Default for DEVICEDUMP_SECTION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DEVICEDUMP_STORAGEDEVICE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DEVICEDUMP_STORAGESTACK_PUBLIC_DUMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DEVICEDUMP_STORAGESTACK_PUBLIC_STATE_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DEVICEDUMP_STRUCTURE_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DEVICEDUMP_SUBSECTION_POINTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DEVICE_COPY_OFFLOAD_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_COPY_OFFLOAD_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.MaximumTokenLifetime == other.MaximumTokenLifetime && self.DefaultTokenLifetime == other.DefaultTokenLifetime && self.MaximumTransferSize == other.MaximumTransferSize && self.OptimalTransferCount == other.OptimalTransferCount && self.MaximumDataDescriptors == other.MaximumDataDescriptors && self.MaximumTransferLengthPerDescriptor == other.MaximumTransferLengthPerDescriptor && self.OptimalTransferLengthPerDescriptor == other.OptimalTransferLengthPerDescriptor && self.OptimalTransferLengthGranularity == other.OptimalTransferLengthGranularity && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for DEVICE_COPY_OFFLOAD_DESCRIPTOR {}
impl ::core::fmt::Debug for DEVICE_COPY_OFFLOAD_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_COPY_OFFLOAD_DESCRIPTOR")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("MaximumTokenLifetime", &self.MaximumTokenLifetime)
            .field("DefaultTokenLifetime", &self.DefaultTokenLifetime)
            .field("MaximumTransferSize", &self.MaximumTransferSize)
            .field("OptimalTransferCount", &self.OptimalTransferCount)
            .field("MaximumDataDescriptors", &self.MaximumDataDescriptors)
            .field("MaximumTransferLengthPerDescriptor", &self.MaximumTransferLengthPerDescriptor)
            .field("OptimalTransferLengthPerDescriptor", &self.OptimalTransferLengthPerDescriptor)
            .field("OptimalTransferLengthGranularity", &self.OptimalTransferLengthGranularity)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::default::Default for DEVICE_DATA_SET_LBP_STATE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DATA_SET_LBP_STATE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Flags == other.Flags && self.OutputVersion == other.OutputVersion
    }
}
impl ::core::cmp::Eq for DEVICE_DATA_SET_LBP_STATE_PARAMETERS {}
impl ::core::fmt::Debug for DEVICE_DATA_SET_LBP_STATE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DATA_SET_LBP_STATE_PARAMETERS").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("OutputVersion", &self.OutputVersion).finish()
    }
}
impl ::core::default::Default for DEVICE_DATA_SET_LB_PROVISIONING_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DATA_SET_LB_PROVISIONING_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.SlabSizeInBytes == other.SlabSizeInBytes && self.SlabOffsetDeltaInBytes == other.SlabOffsetDeltaInBytes && self.SlabAllocationBitMapBitCount == other.SlabAllocationBitMapBitCount && self.SlabAllocationBitMapLength == other.SlabAllocationBitMapLength && self.SlabAllocationBitMap == other.SlabAllocationBitMap
    }
}
impl ::core::cmp::Eq for DEVICE_DATA_SET_LB_PROVISIONING_STATE {}
impl ::core::fmt::Debug for DEVICE_DATA_SET_LB_PROVISIONING_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DATA_SET_LB_PROVISIONING_STATE").field("Size", &self.Size).field("Version", &self.Version).field("SlabSizeInBytes", &self.SlabSizeInBytes).field("SlabOffsetDeltaInBytes", &self.SlabOffsetDeltaInBytes).field("SlabAllocationBitMapBitCount", &self.SlabAllocationBitMapBitCount).field("SlabAllocationBitMapLength", &self.SlabAllocationBitMapLength).field("SlabAllocationBitMap", &self.SlabAllocationBitMap).finish()
    }
}
impl ::core::default::Default for DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self.SlabSizeInBytes == other.SlabSizeInBytes && self.SlabOffsetDeltaInBytes == other.SlabOffsetDeltaInBytes && self.SlabAllocationBitMapBitCount == other.SlabAllocationBitMapBitCount && self.SlabAllocationBitMapLength == other.SlabAllocationBitMapLength && self.SlabAllocationBitMap == other.SlabAllocationBitMap
    }
}
impl ::core::cmp::Eq for DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {}
impl ::core::fmt::Debug for DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DATA_SET_LB_PROVISIONING_STATE_V2").field("Size", &self.Size).field("Version", &self.Version).field("SlabSizeInBytes", &self.SlabSizeInBytes).field("SlabOffsetDeltaInBytes", &self.SlabOffsetDeltaInBytes).field("SlabAllocationBitMapBitCount", &self.SlabAllocationBitMapBitCount).field("SlabAllocationBitMapLength", &self.SlabAllocationBitMapLength).field("SlabAllocationBitMap", &self.SlabAllocationBitMap).finish()
    }
}
impl ::core::default::Default for DEVICE_DATA_SET_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DATA_SET_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartingOffset == other.StartingOffset && self.LengthInBytes == other.LengthInBytes
    }
}
impl ::core::cmp::Eq for DEVICE_DATA_SET_RANGE {}
impl ::core::fmt::Debug for DEVICE_DATA_SET_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DATA_SET_RANGE").field("StartingOffset", &self.StartingOffset).field("LengthInBytes", &self.LengthInBytes).finish()
    }
}
impl ::core::default::Default for DEVICE_DATA_SET_REPAIR_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DATA_SET_REPAIR_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.ParityExtent == other.ParityExtent
    }
}
impl ::core::cmp::Eq for DEVICE_DATA_SET_REPAIR_OUTPUT {}
impl ::core::fmt::Debug for DEVICE_DATA_SET_REPAIR_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DATA_SET_REPAIR_OUTPUT").field("ParityExtent", &self.ParityExtent).finish()
    }
}
impl ::core::default::Default for DEVICE_DATA_SET_REPAIR_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DATA_SET_REPAIR_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfRepairCopies == other.NumberOfRepairCopies && self.SourceCopy == other.SourceCopy && self.RepairCopies == other.RepairCopies
    }
}
impl ::core::cmp::Eq for DEVICE_DATA_SET_REPAIR_PARAMETERS {}
impl ::core::fmt::Debug for DEVICE_DATA_SET_REPAIR_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DATA_SET_REPAIR_PARAMETERS").field("NumberOfRepairCopies", &self.NumberOfRepairCopies).field("SourceCopy", &self.SourceCopy).field("RepairCopies", &self.RepairCopies).finish()
    }
}
impl ::core::default::Default for DEVICE_DATA_SET_SCRUB_EX_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DATA_SET_SCRUB_EX_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.BytesProcessed == other.BytesProcessed && self.BytesRepaired == other.BytesRepaired && self.BytesFailed == other.BytesFailed && self.ParityExtent == other.ParityExtent && self.BytesScrubbed == other.BytesScrubbed
    }
}
impl ::core::cmp::Eq for DEVICE_DATA_SET_SCRUB_EX_OUTPUT {}
impl ::core::fmt::Debug for DEVICE_DATA_SET_SCRUB_EX_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DATA_SET_SCRUB_EX_OUTPUT").field("BytesProcessed", &self.BytesProcessed).field("BytesRepaired", &self.BytesRepaired).field("BytesFailed", &self.BytesFailed).field("ParityExtent", &self.ParityExtent).field("BytesScrubbed", &self.BytesScrubbed).finish()
    }
}
impl ::core::default::Default for DEVICE_DATA_SET_SCRUB_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DATA_SET_SCRUB_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.BytesProcessed == other.BytesProcessed && self.BytesRepaired == other.BytesRepaired && self.BytesFailed == other.BytesFailed
    }
}
impl ::core::cmp::Eq for DEVICE_DATA_SET_SCRUB_OUTPUT {}
impl ::core::fmt::Debug for DEVICE_DATA_SET_SCRUB_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DATA_SET_SCRUB_OUTPUT").field("BytesProcessed", &self.BytesProcessed).field("BytesRepaired", &self.BytesRepaired).field("BytesFailed", &self.BytesFailed).finish()
    }
}
impl ::core::default::Default for DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.TopologyRangeBytes == other.TopologyRangeBytes && self.TopologyId == other.TopologyId
    }
}
impl ::core::cmp::Eq for DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT {}
impl ::core::fmt::Debug for DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DATA_SET_TOPOLOGY_ID_QUERY_OUTPUT").field("TopologyRangeBytes", &self.TopologyRangeBytes).field("TopologyId", &self.TopologyId).finish()
    }
}
impl ::core::default::Default for DEVICE_DSM_CONVERSION_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DSM_CONVERSION_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Source == other.Source
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_CONVERSION_OUTPUT {}
impl ::core::fmt::Debug for DEVICE_DSM_CONVERSION_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_CONVERSION_OUTPUT").field("Version", &self.Version).field("Source", &self.Source).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICE_DSM_DEFINITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVICE_DSM_DEFINITION {
    fn eq(&self, other: &Self) -> bool {
        self.Action == other.Action && self.SingleRange == other.SingleRange && self.ParameterBlockAlignment == other.ParameterBlockAlignment && self.ParameterBlockLength == other.ParameterBlockLength && self.HasOutput == other.HasOutput && self.OutputBlockAlignment == other.OutputBlockAlignment && self.OutputBlockLength == other.OutputBlockLength
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVICE_DSM_DEFINITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVICE_DSM_DEFINITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_DEFINITION").field("Action", &self.Action).field("SingleRange", &self.SingleRange).field("ParameterBlockAlignment", &self.ParameterBlockAlignment).field("ParameterBlockLength", &self.ParameterBlockLength).field("HasOutput", &self.HasOutput).field("OutputBlockAlignment", &self.OutputBlockAlignment).field("OutputBlockLength", &self.OutputBlockLength).finish()
    }
}
impl ::core::default::Default for DEVICE_DSM_FREE_SPACE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DSM_FREE_SPACE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.FreeSpace == other.FreeSpace
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_FREE_SPACE_OUTPUT {}
impl ::core::fmt::Debug for DEVICE_DSM_FREE_SPACE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_FREE_SPACE_OUTPUT").field("Version", &self.Version).field("FreeSpace", &self.FreeSpace).finish()
    }
}
impl ::core::default::Default for DEVICE_DSM_LOST_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DSM_LOST_QUERY_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Alignment == other.Alignment && self.NumberOfBits == other.NumberOfBits && self.BitMap == other.BitMap
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_LOST_QUERY_OUTPUT {}
impl ::core::fmt::Debug for DEVICE_DSM_LOST_QUERY_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_LOST_QUERY_OUTPUT").field("Version", &self.Version).field("Size", &self.Size).field("Alignment", &self.Alignment).field("NumberOfBits", &self.NumberOfBits).field("BitMap", &self.BitMap).finish()
    }
}
impl ::core::default::Default for DEVICE_DSM_LOST_QUERY_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DSM_LOST_QUERY_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Granularity == other.Granularity
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_LOST_QUERY_PARAMETERS {}
impl ::core::fmt::Debug for DEVICE_DSM_LOST_QUERY_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_LOST_QUERY_PARAMETERS").field("Version", &self.Version).field("Granularity", &self.Granularity).finish()
    }
}
impl ::core::default::Default for DEVICE_DSM_NOTIFICATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DSM_NOTIFICATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Flags == other.Flags && self.NumFileTypeIDs == other.NumFileTypeIDs && self.FileTypeID == other.FileTypeID
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_NOTIFICATION_PARAMETERS {}
impl ::core::fmt::Debug for DEVICE_DSM_NOTIFICATION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_NOTIFICATION_PARAMETERS").field("Size", &self.Size).field("Flags", &self.Flags).field("NumFileTypeIDs", &self.NumFileTypeIDs).field("FileTypeID", &self.FileTypeID).finish()
    }
}
impl ::core::default::Default for DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.TargetPriority == other.TargetPriority && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS {}
impl ::core::fmt::Debug for DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_NVCACHE_CHANGE_PRIORITY_PARAMETERS").field("Size", &self.Size).field("TargetPriority", &self.TargetPriority).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for DEVICE_DSM_OFFLOAD_READ_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DSM_OFFLOAD_READ_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.TimeToLive == other.TimeToLive && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_OFFLOAD_READ_PARAMETERS {}
impl ::core::fmt::Debug for DEVICE_DSM_OFFLOAD_READ_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_OFFLOAD_READ_PARAMETERS").field("Flags", &self.Flags).field("TimeToLive", &self.TimeToLive).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for DEVICE_DSM_OFFLOAD_WRITE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.TotalNumberOfRanges == other.TotalNumberOfRanges && self.NumberOfRangesReturned == other.NumberOfRangesReturned && self.Ranges == other.Ranges
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {}
impl ::core::fmt::Debug for DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_PHYSICAL_ADDRESSES_OUTPUT").field("Version", &self.Version).field("Flags", &self.Flags).field("TotalNumberOfRanges", &self.TotalNumberOfRanges).field("NumberOfRangesReturned", &self.NumberOfRangesReturned).field("Ranges", &self.Ranges).finish()
    }
}
impl ::core::default::Default for DEVICE_DSM_RANGE_ERROR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICE_DSM_REPORT_ZONES_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVICE_DSM_REPORT_ZONES_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.ZoneCount == other.ZoneCount && self.Attributes == other.Attributes && self.Reserved0 == other.Reserved0 && self.ZoneDescriptors == other.ZoneDescriptors
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVICE_DSM_REPORT_ZONES_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVICE_DSM_REPORT_ZONES_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_REPORT_ZONES_DATA").field("Size", &self.Size).field("ZoneCount", &self.ZoneCount).field("Attributes", &self.Attributes).field("Reserved0", &self.Reserved0).field("ZoneDescriptors", &self.ZoneDescriptors).finish()
    }
}
impl ::core::default::Default for DEVICE_DSM_REPORT_ZONES_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DSM_REPORT_ZONES_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.ReportOption == other.ReportOption && self.Partial == other.Partial && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_REPORT_ZONES_PARAMETERS {}
impl ::core::fmt::Debug for DEVICE_DSM_REPORT_ZONES_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_REPORT_ZONES_PARAMETERS").field("Size", &self.Size).field("ReportOption", &self.ReportOption).field("Partial", &self.Partial).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for DEVICE_DSM_TIERING_QUERY_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DSM_TIERING_QUERY_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Flags == other.Flags && self.NumberOfTierIds == other.NumberOfTierIds && self.TierIds == other.TierIds
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_TIERING_QUERY_INPUT {}
impl ::core::fmt::Debug for DEVICE_DSM_TIERING_QUERY_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_TIERING_QUERY_INPUT").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("NumberOfTierIds", &self.NumberOfTierIds).field("TierIds", &self.TierIds).finish()
    }
}
impl ::core::default::Default for DEVICE_DSM_TIERING_QUERY_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_DSM_TIERING_QUERY_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Flags == other.Flags && self.Reserved == other.Reserved && self.Alignment == other.Alignment && self.TotalNumberOfRegions == other.TotalNumberOfRegions && self.NumberOfRegionsReturned == other.NumberOfRegionsReturned && self.Regions == other.Regions
    }
}
impl ::core::cmp::Eq for DEVICE_DSM_TIERING_QUERY_OUTPUT {}
impl ::core::fmt::Debug for DEVICE_DSM_TIERING_QUERY_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_DSM_TIERING_QUERY_OUTPUT").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Alignment", &self.Alignment).field("TotalNumberOfRegions", &self.TotalNumberOfRegions).field("NumberOfRegionsReturned", &self.NumberOfRegionsReturned).field("Regions", &self.Regions).finish()
    }
}
impl ::core::default::Default for DEVICE_INTERNAL_STATUS_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_INTERNAL_STATUS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.T10VendorId == other.T10VendorId && self.DataSet1Length == other.DataSet1Length && self.DataSet2Length == other.DataSet2Length && self.DataSet3Length == other.DataSet3Length && self.DataSet4Length == other.DataSet4Length && self.StatusDataVersion == other.StatusDataVersion && self.Reserved == other.Reserved && self.ReasonIdentifier == other.ReasonIdentifier && self.StatusDataLength == other.StatusDataLength && self.StatusData == other.StatusData
    }
}
impl ::core::cmp::Eq for DEVICE_INTERNAL_STATUS_DATA {}
impl ::core::fmt::Debug for DEVICE_INTERNAL_STATUS_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_INTERNAL_STATUS_DATA")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("T10VendorId", &self.T10VendorId)
            .field("DataSet1Length", &self.DataSet1Length)
            .field("DataSet2Length", &self.DataSet2Length)
            .field("DataSet3Length", &self.DataSet3Length)
            .field("DataSet4Length", &self.DataSet4Length)
            .field("StatusDataVersion", &self.StatusDataVersion)
            .field("Reserved", &self.Reserved)
            .field("ReasonIdentifier", &self.ReasonIdentifier)
            .field("StatusDataLength", &self.StatusDataLength)
            .field("StatusData", &self.StatusData)
            .finish()
    }
}
impl ::core::default::Default for DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICE_INTERNAL_STATUS_DATA_REQUEST_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEVICE_INTERNAL_STATUS_DATA_SET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVICE_INTERNAL_STATUS_DATA_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICE_INTERNAL_STATUS_DATA_SET").field(&self.0).finish()
    }
}
impl ::core::default::Default for DEVICE_LB_PROVISIONING_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_LB_PROVISIONING_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self._bitfield == other._bitfield && self.Reserved1 == other.Reserved1 && self.OptimalUnmapGranularity == other.OptimalUnmapGranularity && self.UnmapGranularityAlignment == other.UnmapGranularityAlignment && self.MaxUnmapLbaCount == other.MaxUnmapLbaCount && self.MaxUnmapBlockDescriptorCount == other.MaxUnmapBlockDescriptorCount
    }
}
impl ::core::cmp::Eq for DEVICE_LB_PROVISIONING_DESCRIPTOR {}
impl ::core::fmt::Debug for DEVICE_LB_PROVISIONING_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_LB_PROVISIONING_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("_bitfield", &self._bitfield).field("Reserved1", &self.Reserved1).field("OptimalUnmapGranularity", &self.OptimalUnmapGranularity).field("UnmapGranularityAlignment", &self.UnmapGranularityAlignment).field("MaxUnmapLbaCount", &self.MaxUnmapLbaCount).field("MaxUnmapBlockDescriptorCount", &self.MaxUnmapBlockDescriptorCount).finish()
    }
}
impl ::core::default::Default for DEVICE_LOCATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DEVICE_MANAGE_DATA_SET_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_MANAGE_DATA_SET_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Action == other.Action && self.Flags == other.Flags && self.ParameterBlockOffset == other.ParameterBlockOffset && self.ParameterBlockLength == other.ParameterBlockLength && self.DataSetRangesOffset == other.DataSetRangesOffset && self.DataSetRangesLength == other.DataSetRangesLength
    }
}
impl ::core::cmp::Eq for DEVICE_MANAGE_DATA_SET_ATTRIBUTES {}
impl ::core::fmt::Debug for DEVICE_MANAGE_DATA_SET_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_MANAGE_DATA_SET_ATTRIBUTES").field("Size", &self.Size).field("Action", &self.Action).field("Flags", &self.Flags).field("ParameterBlockOffset", &self.ParameterBlockOffset).field("ParameterBlockLength", &self.ParameterBlockLength).field("DataSetRangesOffset", &self.DataSetRangesOffset).field("DataSetRangesLength", &self.DataSetRangesLength).finish()
    }
}
impl ::core::default::Default for DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Action == other.Action && self.Flags == other.Flags && self.OperationStatus == other.OperationStatus && self.ExtendedError == other.ExtendedError && self.TargetDetailedError == other.TargetDetailedError && self.ReservedStatus == other.ReservedStatus && self.OutputBlockOffset == other.OutputBlockOffset && self.OutputBlockLength == other.OutputBlockLength
    }
}
impl ::core::cmp::Eq for DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT {}
impl ::core::fmt::Debug for DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_MANAGE_DATA_SET_ATTRIBUTES_OUTPUT").field("Size", &self.Size).field("Action", &self.Action).field("Flags", &self.Flags).field("OperationStatus", &self.OperationStatus).field("ExtendedError", &self.ExtendedError).field("TargetDetailedError", &self.TargetDetailedError).field("ReservedStatus", &self.ReservedStatus).field("OutputBlockOffset", &self.OutputBlockOffset).field("OutputBlockLength", &self.OutputBlockLength).finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for DEVICE_MEDIA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICE_POWER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVICE_POWER_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.DeviceAttentionSupported == other.DeviceAttentionSupported && self.AsynchronousNotificationSupported == other.AsynchronousNotificationSupported && self.IdlePowerManagementEnabled == other.IdlePowerManagementEnabled && self.D3ColdEnabled == other.D3ColdEnabled && self.D3ColdSupported == other.D3ColdSupported && self.NoVerifyDuringIdlePower == other.NoVerifyDuringIdlePower && self.Reserved == other.Reserved && self.IdleTimeoutInMS == other.IdleTimeoutInMS
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVICE_POWER_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVICE_POWER_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_POWER_DESCRIPTOR")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("DeviceAttentionSupported", &self.DeviceAttentionSupported)
            .field("AsynchronousNotificationSupported", &self.AsynchronousNotificationSupported)
            .field("IdlePowerManagementEnabled", &self.IdlePowerManagementEnabled)
            .field("D3ColdEnabled", &self.D3ColdEnabled)
            .field("D3ColdSupported", &self.D3ColdSupported)
            .field("NoVerifyDuringIdlePower", &self.NoVerifyDuringIdlePower)
            .field("Reserved", &self.Reserved)
            .field("IdleTimeoutInMS", &self.IdleTimeoutInMS)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICE_SEEK_PENALTY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVICE_SEEK_PENALTY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.IncursSeekPenalty == other.IncursSeekPenalty
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVICE_SEEK_PENALTY_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVICE_SEEK_PENALTY_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_SEEK_PENALTY_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("IncursSeekPenalty", &self.IncursSeekPenalty).finish()
    }
}
impl ::core::default::Default for DEVICE_STORAGE_ADDRESS_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DEVICE_STORAGE_ADDRESS_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartAddress == other.StartAddress && self.LengthInBytes == other.LengthInBytes
    }
}
impl ::core::cmp::Eq for DEVICE_STORAGE_ADDRESS_RANGE {}
impl ::core::fmt::Debug for DEVICE_STORAGE_ADDRESS_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_STORAGE_ADDRESS_RANGE").field("StartAddress", &self.StartAddress).field("LengthInBytes", &self.LengthInBytes).finish()
    }
}
impl ::core::default::Default for DEVICE_STORAGE_RANGE_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICE_TRIM_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVICE_TRIM_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.TrimEnabled == other.TrimEnabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVICE_TRIM_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVICE_TRIM_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_TRIM_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("TrimEnabled", &self.TrimEnabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DEVICE_WRITE_AGGREGATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DEVICE_WRITE_AGGREGATION_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.BenefitsFromWriteAggregation == other.BenefitsFromWriteAggregation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DEVICE_WRITE_AGGREGATION_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DEVICE_WRITE_AGGREGATION_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVICE_WRITE_AGGREGATION_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("BenefitsFromWriteAggregation", &self.BenefitsFromWriteAggregation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISK_CACHE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DISK_CACHE_RETENTION_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DISK_CACHE_RETENTION_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISK_CACHE_RETENTION_PRIORITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for DISK_CONTROLLER_NUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISK_CONTROLLER_NUMBER {
    fn eq(&self, other: &Self) -> bool {
        self.ControllerNumber == other.ControllerNumber && self.DiskNumber == other.DiskNumber
    }
}
impl ::core::cmp::Eq for DISK_CONTROLLER_NUMBER {}
impl ::core::fmt::Debug for DISK_CONTROLLER_NUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_CONTROLLER_NUMBER").field("ControllerNumber", &self.ControllerNumber).field("DiskNumber", &self.DiskNumber).finish()
    }
}
impl ::core::default::Default for DISK_DETECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DISK_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISK_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.DiskNumber == other.DiskNumber && self.StartingOffset == other.StartingOffset && self.ExtentLength == other.ExtentLength
    }
}
impl ::core::cmp::Eq for DISK_EXTENT {}
impl ::core::fmt::Debug for DISK_EXTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_EXTENT").field("DiskNumber", &self.DiskNumber).field("StartingOffset", &self.StartingOffset).field("ExtentLength", &self.ExtentLength).finish()
    }
}
impl ::core::default::Default for DISK_EX_INT13_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISK_EX_INT13_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ExBufferSize == other.ExBufferSize && self.ExFlags == other.ExFlags && self.ExCylinders == other.ExCylinders && self.ExHeads == other.ExHeads && self.ExSectorsPerTrack == other.ExSectorsPerTrack && self.ExSectorsPerDrive == other.ExSectorsPerDrive && self.ExSectorSize == other.ExSectorSize && self.ExReserved == other.ExReserved
    }
}
impl ::core::cmp::Eq for DISK_EX_INT13_INFO {}
impl ::core::fmt::Debug for DISK_EX_INT13_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_EX_INT13_INFO").field("ExBufferSize", &self.ExBufferSize).field("ExFlags", &self.ExFlags).field("ExCylinders", &self.ExCylinders).field("ExHeads", &self.ExHeads).field("ExSectorsPerTrack", &self.ExSectorsPerTrack).field("ExSectorsPerDrive", &self.ExSectorsPerDrive).field("ExSectorSize", &self.ExSectorSize).field("ExReserved", &self.ExReserved).finish()
    }
}
impl ::core::default::Default for DISK_GEOMETRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISK_GEOMETRY {
    fn eq(&self, other: &Self) -> bool {
        self.Cylinders == other.Cylinders && self.MediaType == other.MediaType && self.TracksPerCylinder == other.TracksPerCylinder && self.SectorsPerTrack == other.SectorsPerTrack && self.BytesPerSector == other.BytesPerSector
    }
}
impl ::core::cmp::Eq for DISK_GEOMETRY {}
impl ::core::fmt::Debug for DISK_GEOMETRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_GEOMETRY").field("Cylinders", &self.Cylinders).field("MediaType", &self.MediaType).field("TracksPerCylinder", &self.TracksPerCylinder).field("SectorsPerTrack", &self.SectorsPerTrack).field("BytesPerSector", &self.BytesPerSector).finish()
    }
}
impl ::core::default::Default for DISK_GEOMETRY_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISK_GEOMETRY_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Geometry == other.Geometry && self.DiskSize == other.DiskSize && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for DISK_GEOMETRY_EX {}
impl ::core::fmt::Debug for DISK_GEOMETRY_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_GEOMETRY_EX").field("Geometry", &self.Geometry).field("DiskSize", &self.DiskSize).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for DISK_GROW_PARTITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISK_GROW_PARTITION {
    fn eq(&self, other: &Self) -> bool {
        self.PartitionNumber == other.PartitionNumber && self.BytesToGrow == other.BytesToGrow
    }
}
impl ::core::cmp::Eq for DISK_GROW_PARTITION {}
impl ::core::fmt::Debug for DISK_GROW_PARTITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_GROW_PARTITION").field("PartitionNumber", &self.PartitionNumber).field("BytesToGrow", &self.BytesToGrow).finish()
    }
}
impl ::core::default::Default for DISK_HISTOGRAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISK_HISTOGRAM {
    fn eq(&self, other: &Self) -> bool {
        self.DiskSize == other.DiskSize && self.Start == other.Start && self.End == other.End && self.Average == other.Average && self.AverageRead == other.AverageRead && self.AverageWrite == other.AverageWrite && self.Granularity == other.Granularity && self.Size == other.Size && self.ReadCount == other.ReadCount && self.WriteCount == other.WriteCount && self.Histogram == other.Histogram
    }
}
impl ::core::cmp::Eq for DISK_HISTOGRAM {}
impl ::core::fmt::Debug for DISK_HISTOGRAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_HISTOGRAM").field("DiskSize", &self.DiskSize).field("Start", &self.Start).field("End", &self.End).field("Average", &self.Average).field("AverageRead", &self.AverageRead).field("AverageWrite", &self.AverageWrite).field("Granularity", &self.Granularity).field("Size", &self.Size).field("ReadCount", &self.ReadCount).field("WriteCount", &self.WriteCount).field("Histogram", &self.Histogram).finish()
    }
}
impl ::core::default::Default for DISK_INT13_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISK_INT13_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DriveSelect == other.DriveSelect && self.MaxCylinders == other.MaxCylinders && self.SectorsPerTrack == other.SectorsPerTrack && self.MaxHeads == other.MaxHeads && self.NumberDrives == other.NumberDrives
    }
}
impl ::core::cmp::Eq for DISK_INT13_INFO {}
impl ::core::fmt::Debug for DISK_INT13_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_INT13_INFO").field("DriveSelect", &self.DriveSelect).field("MaxCylinders", &self.MaxCylinders).field("SectorsPerTrack", &self.SectorsPerTrack).field("MaxHeads", &self.MaxHeads).field("NumberDrives", &self.NumberDrives).finish()
    }
}
impl ::core::default::Default for DISK_LOGGING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISK_LOGGING {
    fn eq(&self, other: &Self) -> bool {
        self.Function == other.Function && self.BufferAddress == other.BufferAddress && self.BufferSize == other.BufferSize
    }
}
impl ::core::cmp::Eq for DISK_LOGGING {}
impl ::core::fmt::Debug for DISK_LOGGING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_LOGGING").field("Function", &self.Function).field("BufferAddress", &self.BufferAddress).field("BufferSize", &self.BufferSize).finish()
    }
}
impl ::core::default::Default for DISK_PARTITION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DISK_PERFORMANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DISK_PERFORMANCE {
    fn eq(&self, other: &Self) -> bool {
        self.BytesRead == other.BytesRead && self.BytesWritten == other.BytesWritten && self.ReadTime == other.ReadTime && self.WriteTime == other.WriteTime && self.IdleTime == other.IdleTime && self.ReadCount == other.ReadCount && self.WriteCount == other.WriteCount && self.QueueDepth == other.QueueDepth && self.SplitCount == other.SplitCount && self.QueryTime == other.QueryTime && self.StorageDeviceNumber == other.StorageDeviceNumber && self.StorageManagerName == other.StorageManagerName
    }
}
impl ::core::cmp::Eq for DISK_PERFORMANCE {}
impl ::core::fmt::Debug for DISK_PERFORMANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_PERFORMANCE")
            .field("BytesRead", &self.BytesRead)
            .field("BytesWritten", &self.BytesWritten)
            .field("ReadTime", &self.ReadTime)
            .field("WriteTime", &self.WriteTime)
            .field("IdleTime", &self.IdleTime)
            .field("ReadCount", &self.ReadCount)
            .field("WriteCount", &self.WriteCount)
            .field("QueueDepth", &self.QueueDepth)
            .field("SplitCount", &self.SplitCount)
            .field("QueryTime", &self.QueryTime)
            .field("StorageDeviceNumber", &self.StorageDeviceNumber)
            .field("StorageManagerName", &self.StorageManagerName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISK_RECORD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISK_RECORD {
    fn eq(&self, other: &Self) -> bool {
        self.ByteOffset == other.ByteOffset && self.StartTime == other.StartTime && self.EndTime == other.EndTime && self.VirtualAddress == other.VirtualAddress && self.NumberOfBytes == other.NumberOfBytes && self.DeviceNumber == other.DeviceNumber && self.ReadRequest == other.ReadRequest
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISK_RECORD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISK_RECORD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISK_RECORD").field("ByteOffset", &self.ByteOffset).field("StartTime", &self.StartTime).field("EndTime", &self.EndTime).field("VirtualAddress", &self.VirtualAddress).field("NumberOfBytes", &self.NumberOfBytes).field("DeviceNumber", &self.DeviceNumber).field("ReadRequest", &self.ReadRequest).finish()
    }
}
impl ::core::default::Default for DRIVERSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRIVE_LAYOUT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRIVE_LAYOUT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.PartitionCount == other.PartitionCount && self.Signature == other.Signature && self.PartitionEntry == other.PartitionEntry
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRIVE_LAYOUT_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DRIVE_LAYOUT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVE_LAYOUT_INFORMATION").field("PartitionCount", &self.PartitionCount).field("Signature", &self.Signature).field("PartitionEntry", &self.PartitionEntry).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRIVE_LAYOUT_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for DRIVE_LAYOUT_INFORMATION_GPT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRIVE_LAYOUT_INFORMATION_GPT {
    fn eq(&self, other: &Self) -> bool {
        self.DiskId == other.DiskId && self.StartingUsableOffset == other.StartingUsableOffset && self.UsableLength == other.UsableLength && self.MaxPartitionCount == other.MaxPartitionCount
    }
}
impl ::core::cmp::Eq for DRIVE_LAYOUT_INFORMATION_GPT {}
impl ::core::fmt::Debug for DRIVE_LAYOUT_INFORMATION_GPT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVE_LAYOUT_INFORMATION_GPT").field("DiskId", &self.DiskId).field("StartingUsableOffset", &self.StartingUsableOffset).field("UsableLength", &self.UsableLength).field("MaxPartitionCount", &self.MaxPartitionCount).finish()
    }
}
impl ::core::default::Default for DRIVE_LAYOUT_INFORMATION_MBR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DRIVE_LAYOUT_INFORMATION_MBR {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature && self.CheckSum == other.CheckSum
    }
}
impl ::core::cmp::Eq for DRIVE_LAYOUT_INFORMATION_MBR {}
impl ::core::fmt::Debug for DRIVE_LAYOUT_INFORMATION_MBR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRIVE_LAYOUT_INFORMATION_MBR").field("Signature", &self.Signature).field("CheckSum", &self.CheckSum).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DUPLICATE_EXTENTS_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DUPLICATE_EXTENTS_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.FileHandle == other.FileHandle && self.SourceFileOffset == other.SourceFileOffset && self.TargetFileOffset == other.TargetFileOffset && self.ByteCount == other.ByteCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DUPLICATE_EXTENTS_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DUPLICATE_EXTENTS_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DUPLICATE_EXTENTS_DATA").field("FileHandle", &self.FileHandle).field("SourceFileOffset", &self.SourceFileOffset).field("TargetFileOffset", &self.TargetFileOffset).field("ByteCount", &self.ByteCount).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DUPLICATE_EXTENTS_DATA32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for DUPLICATE_EXTENTS_DATA32 {
    fn eq(&self, other: &Self) -> bool {
        self.FileHandle == other.FileHandle && self.SourceFileOffset == other.SourceFileOffset && self.TargetFileOffset == other.TargetFileOffset && self.ByteCount == other.ByteCount
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for DUPLICATE_EXTENTS_DATA32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for DUPLICATE_EXTENTS_DATA32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DUPLICATE_EXTENTS_DATA32").field("FileHandle", &self.FileHandle).field("SourceFileOffset", &self.SourceFileOffset).field("TargetFileOffset", &self.TargetFileOffset).field("ByteCount", &self.ByteCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DUPLICATE_EXTENTS_DATA_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DUPLICATE_EXTENTS_DATA_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.FileHandle == other.FileHandle && self.SourceFileOffset == other.SourceFileOffset && self.TargetFileOffset == other.TargetFileOffset && self.ByteCount == other.ByteCount && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DUPLICATE_EXTENTS_DATA_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DUPLICATE_EXTENTS_DATA_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DUPLICATE_EXTENTS_DATA_EX").field("Size", &self.Size).field("FileHandle", &self.FileHandle).field("SourceFileOffset", &self.SourceFileOffset).field("TargetFileOffset", &self.TargetFileOffset).field("ByteCount", &self.ByteCount).field("Flags", &self.Flags).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DUPLICATE_EXTENTS_DATA_EX32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for DUPLICATE_EXTENTS_DATA_EX32 {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.FileHandle == other.FileHandle && self.SourceFileOffset == other.SourceFileOffset && self.TargetFileOffset == other.TargetFileOffset && self.ByteCount == other.ByteCount && self.Flags == other.Flags
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for DUPLICATE_EXTENTS_DATA_EX32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for DUPLICATE_EXTENTS_DATA_EX32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DUPLICATE_EXTENTS_DATA_EX32").field("Size", &self.Size).field("FileHandle", &self.FileHandle).field("SourceFileOffset", &self.SourceFileOffset).field("TargetFileOffset", &self.TargetFileOffset).field("ByteCount", &self.ByteCount).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for DUPLICATE_EXTENTS_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DUPLICATE_EXTENTS_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DUPLICATE_EXTENTS_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ELEMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for ELEMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ELEMENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for ENCRYPTED_DATA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENCRYPTED_DATA_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.StartingFileOffset == other.StartingFileOffset && self.OutputBufferOffset == other.OutputBufferOffset && self.BytesWithinFileSize == other.BytesWithinFileSize && self.BytesWithinValidDataLength == other.BytesWithinValidDataLength && self.CompressionFormat == other.CompressionFormat && self.DataUnitShift == other.DataUnitShift && self.ChunkShift == other.ChunkShift && self.ClusterShift == other.ClusterShift && self.EncryptionFormat == other.EncryptionFormat && self.NumberOfDataBlocks == other.NumberOfDataBlocks && self.DataBlockSize == other.DataBlockSize
    }
}
impl ::core::cmp::Eq for ENCRYPTED_DATA_INFO {}
impl ::core::fmt::Debug for ENCRYPTED_DATA_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTED_DATA_INFO")
            .field("StartingFileOffset", &self.StartingFileOffset)
            .field("OutputBufferOffset", &self.OutputBufferOffset)
            .field("BytesWithinFileSize", &self.BytesWithinFileSize)
            .field("BytesWithinValidDataLength", &self.BytesWithinValidDataLength)
            .field("CompressionFormat", &self.CompressionFormat)
            .field("DataUnitShift", &self.DataUnitShift)
            .field("ChunkShift", &self.ChunkShift)
            .field("ClusterShift", &self.ClusterShift)
            .field("EncryptionFormat", &self.EncryptionFormat)
            .field("NumberOfDataBlocks", &self.NumberOfDataBlocks)
            .field("DataBlockSize", &self.DataBlockSize)
            .finish()
    }
}
impl ::core::default::Default for ENCRYPTION_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENCRYPTION_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.EncryptionOperation == other.EncryptionOperation && self.Private == other.Private
    }
}
impl ::core::cmp::Eq for ENCRYPTION_BUFFER {}
impl ::core::fmt::Debug for ENCRYPTION_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_BUFFER").field("EncryptionOperation", &self.EncryptionOperation).field("Private", &self.Private).finish()
    }
}
impl ::core::default::Default for ENCRYPTION_KEY_CTRL_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for ENCRYPTION_KEY_CTRL_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.HeaderSize == other.HeaderSize && self.StructureSize == other.StructureSize && self.KeyOffset == other.KeyOffset && self.KeySize == other.KeySize && self.DplLock == other.DplLock && self.DplUserId == other.DplUserId && self.DplCredentialId == other.DplCredentialId
    }
}
impl ::core::cmp::Eq for ENCRYPTION_KEY_CTRL_INPUT {}
impl ::core::fmt::Debug for ENCRYPTION_KEY_CTRL_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENCRYPTION_KEY_CTRL_INPUT").field("HeaderSize", &self.HeaderSize).field("StructureSize", &self.StructureSize).field("KeyOffset", &self.KeyOffset).field("KeySize", &self.KeySize).field("DplLock", &self.DplLock).field("DplUserId", &self.DplUserId).field("DplCredentialId", &self.DplCredentialId).finish()
    }
}
impl ::core::default::Default for EXFAT_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EXFAT_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.CreateHits == other.CreateHits && self.SuccessfulCreates == other.SuccessfulCreates && self.FailedCreates == other.FailedCreates && self.NonCachedReads == other.NonCachedReads && self.NonCachedReadBytes == other.NonCachedReadBytes && self.NonCachedWrites == other.NonCachedWrites && self.NonCachedWriteBytes == other.NonCachedWriteBytes && self.NonCachedDiskReads == other.NonCachedDiskReads && self.NonCachedDiskWrites == other.NonCachedDiskWrites
    }
}
impl ::core::cmp::Eq for EXFAT_STATISTICS {}
impl ::core::fmt::Debug for EXFAT_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXFAT_STATISTICS")
            .field("CreateHits", &self.CreateHits)
            .field("SuccessfulCreates", &self.SuccessfulCreates)
            .field("FailedCreates", &self.FailedCreates)
            .field("NonCachedReads", &self.NonCachedReads)
            .field("NonCachedReadBytes", &self.NonCachedReadBytes)
            .field("NonCachedWrites", &self.NonCachedWrites)
            .field("NonCachedWriteBytes", &self.NonCachedWriteBytes)
            .field("NonCachedDiskReads", &self.NonCachedDiskReads)
            .field("NonCachedDiskWrites", &self.NonCachedDiskWrites)
            .finish()
    }
}
impl ::core::default::Default for EXTENDED_ENCRYPTED_DATA_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for EXTENDED_ENCRYPTED_DATA_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ExtendedCode == other.ExtendedCode && self.Length == other.Length && self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for EXTENDED_ENCRYPTED_DATA_INFO {}
impl ::core::fmt::Debug for EXTENDED_ENCRYPTED_DATA_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EXTENDED_ENCRYPTED_DATA_INFO").field("ExtendedCode", &self.ExtendedCode).field("Length", &self.Length).field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for FAT_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FAT_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.CreateHits == other.CreateHits && self.SuccessfulCreates == other.SuccessfulCreates && self.FailedCreates == other.FailedCreates && self.NonCachedReads == other.NonCachedReads && self.NonCachedReadBytes == other.NonCachedReadBytes && self.NonCachedWrites == other.NonCachedWrites && self.NonCachedWriteBytes == other.NonCachedWriteBytes && self.NonCachedDiskReads == other.NonCachedDiskReads && self.NonCachedDiskWrites == other.NonCachedDiskWrites
    }
}
impl ::core::cmp::Eq for FAT_STATISTICS {}
impl ::core::fmt::Debug for FAT_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FAT_STATISTICS")
            .field("CreateHits", &self.CreateHits)
            .field("SuccessfulCreates", &self.SuccessfulCreates)
            .field("FailedCreates", &self.FailedCreates)
            .field("NonCachedReads", &self.NonCachedReads)
            .field("NonCachedReadBytes", &self.NonCachedReadBytes)
            .field("NonCachedWrites", &self.NonCachedWrites)
            .field("NonCachedWriteBytes", &self.NonCachedWriteBytes)
            .field("NonCachedDiskReads", &self.NonCachedDiskReads)
            .field("NonCachedDiskWrites", &self.NonCachedDiskWrites)
            .finish()
    }
}
impl ::core::default::Default for FILESYSTEM_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILESYSTEM_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.FileSystemType == other.FileSystemType && self.Version == other.Version && self.SizeOfCompleteStructure == other.SizeOfCompleteStructure && self.UserFileReads == other.UserFileReads && self.UserFileReadBytes == other.UserFileReadBytes && self.UserDiskReads == other.UserDiskReads && self.UserFileWrites == other.UserFileWrites && self.UserFileWriteBytes == other.UserFileWriteBytes && self.UserDiskWrites == other.UserDiskWrites && self.MetaDataReads == other.MetaDataReads && self.MetaDataReadBytes == other.MetaDataReadBytes && self.MetaDataDiskReads == other.MetaDataDiskReads && self.MetaDataWrites == other.MetaDataWrites && self.MetaDataWriteBytes == other.MetaDataWriteBytes && self.MetaDataDiskWrites == other.MetaDataDiskWrites
    }
}
impl ::core::cmp::Eq for FILESYSTEM_STATISTICS {}
impl ::core::fmt::Debug for FILESYSTEM_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILESYSTEM_STATISTICS")
            .field("FileSystemType", &self.FileSystemType)
            .field("Version", &self.Version)
            .field("SizeOfCompleteStructure", &self.SizeOfCompleteStructure)
            .field("UserFileReads", &self.UserFileReads)
            .field("UserFileReadBytes", &self.UserFileReadBytes)
            .field("UserDiskReads", &self.UserDiskReads)
            .field("UserFileWrites", &self.UserFileWrites)
            .field("UserFileWriteBytes", &self.UserFileWriteBytes)
            .field("UserDiskWrites", &self.UserDiskWrites)
            .field("MetaDataReads", &self.MetaDataReads)
            .field("MetaDataReadBytes", &self.MetaDataReadBytes)
            .field("MetaDataDiskReads", &self.MetaDataDiskReads)
            .field("MetaDataWrites", &self.MetaDataWrites)
            .field("MetaDataWriteBytes", &self.MetaDataWriteBytes)
            .field("MetaDataDiskWrites", &self.MetaDataDiskWrites)
            .finish()
    }
}
impl ::core::default::Default for FILESYSTEM_STATISTICS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILESYSTEM_STATISTICS_EX {
    fn eq(&self, other: &Self) -> bool {
        self.FileSystemType == other.FileSystemType && self.Version == other.Version && self.SizeOfCompleteStructure == other.SizeOfCompleteStructure && self.UserFileReads == other.UserFileReads && self.UserFileReadBytes == other.UserFileReadBytes && self.UserDiskReads == other.UserDiskReads && self.UserFileWrites == other.UserFileWrites && self.UserFileWriteBytes == other.UserFileWriteBytes && self.UserDiskWrites == other.UserDiskWrites && self.MetaDataReads == other.MetaDataReads && self.MetaDataReadBytes == other.MetaDataReadBytes && self.MetaDataDiskReads == other.MetaDataDiskReads && self.MetaDataWrites == other.MetaDataWrites && self.MetaDataWriteBytes == other.MetaDataWriteBytes && self.MetaDataDiskWrites == other.MetaDataDiskWrites
    }
}
impl ::core::cmp::Eq for FILESYSTEM_STATISTICS_EX {}
impl ::core::fmt::Debug for FILESYSTEM_STATISTICS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILESYSTEM_STATISTICS_EX")
            .field("FileSystemType", &self.FileSystemType)
            .field("Version", &self.Version)
            .field("SizeOfCompleteStructure", &self.SizeOfCompleteStructure)
            .field("UserFileReads", &self.UserFileReads)
            .field("UserFileReadBytes", &self.UserFileReadBytes)
            .field("UserDiskReads", &self.UserDiskReads)
            .field("UserFileWrites", &self.UserFileWrites)
            .field("UserFileWriteBytes", &self.UserFileWriteBytes)
            .field("UserDiskWrites", &self.UserDiskWrites)
            .field("MetaDataReads", &self.MetaDataReads)
            .field("MetaDataReadBytes", &self.MetaDataReadBytes)
            .field("MetaDataDiskReads", &self.MetaDataDiskReads)
            .field("MetaDataWrites", &self.MetaDataWrites)
            .field("MetaDataWriteBytes", &self.MetaDataWriteBytes)
            .field("MetaDataDiskWrites", &self.MetaDataDiskWrites)
            .finish()
    }
}
impl ::core::default::Default for FILESYSTEM_STATISTICS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILESYSTEM_STATISTICS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILESYSTEM_STATISTICS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILE_ALLOCATED_RANGE_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_ALLOCATED_RANGE_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.FileOffset == other.FileOffset && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for FILE_ALLOCATED_RANGE_BUFFER {}
impl ::core::fmt::Debug for FILE_ALLOCATED_RANGE_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ALLOCATED_RANGE_BUFFER").field("FileOffset", &self.FileOffset).field("Length", &self.Length).finish()
    }
}
impl ::core::default::Default for FILE_DESIRED_STORAGE_CLASS_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_DESIRED_STORAGE_CLASS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Class == other.Class && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for FILE_DESIRED_STORAGE_CLASS_INFORMATION {}
impl ::core::fmt::Debug for FILE_DESIRED_STORAGE_CLASS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_DESIRED_STORAGE_CLASS_INFORMATION").field("Class", &self.Class).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for FILE_FS_PERSISTENT_VOLUME_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_FS_PERSISTENT_VOLUME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeFlags == other.VolumeFlags && self.FlagMask == other.FlagMask && self.Version == other.Version && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for FILE_FS_PERSISTENT_VOLUME_INFORMATION {}
impl ::core::fmt::Debug for FILE_FS_PERSISTENT_VOLUME_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_FS_PERSISTENT_VOLUME_INFORMATION").field("VolumeFlags", &self.VolumeFlags).field("FlagMask", &self.FlagMask).field("Version", &self.Version).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for FILE_INITIATE_REPAIR_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_INITIATE_REPAIR_OUTPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.Hint1 == other.Hint1 && self.Hint2 == other.Hint2 && self.Clsn == other.Clsn && self.Status == other.Status
    }
}
impl ::core::cmp::Eq for FILE_INITIATE_REPAIR_OUTPUT_BUFFER {}
impl ::core::fmt::Debug for FILE_INITIATE_REPAIR_OUTPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_INITIATE_REPAIR_OUTPUT_BUFFER").field("Hint1", &self.Hint1).field("Hint2", &self.Hint2).field("Clsn", &self.Clsn).field("Status", &self.Status).finish()
    }
}
impl ::core::default::Default for FILE_LAYOUT_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_LAYOUT_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.NextFileOffset == other.NextFileOffset && self.Flags == other.Flags && self.FileAttributes == other.FileAttributes && self.FileReferenceNumber == other.FileReferenceNumber && self.FirstNameOffset == other.FirstNameOffset && self.FirstStreamOffset == other.FirstStreamOffset && self.ExtraInfoOffset == other.ExtraInfoOffset && self.ExtraInfoLength == other.ExtraInfoLength
    }
}
impl ::core::cmp::Eq for FILE_LAYOUT_ENTRY {}
impl ::core::fmt::Debug for FILE_LAYOUT_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_LAYOUT_ENTRY").field("Version", &self.Version).field("NextFileOffset", &self.NextFileOffset).field("Flags", &self.Flags).field("FileAttributes", &self.FileAttributes).field("FileReferenceNumber", &self.FileReferenceNumber).field("FirstNameOffset", &self.FirstNameOffset).field("FirstStreamOffset", &self.FirstStreamOffset).field("ExtraInfoOffset", &self.ExtraInfoOffset).field("ExtraInfoLength", &self.ExtraInfoLength).finish()
    }
}
impl ::core::default::Default for FILE_LAYOUT_INFO_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_LAYOUT_INFO_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.BasicInformation == other.BasicInformation && self.OwnerId == other.OwnerId && self.SecurityId == other.SecurityId && self.Usn == other.Usn && self.StorageReserveId == other.StorageReserveId
    }
}
impl ::core::cmp::Eq for FILE_LAYOUT_INFO_ENTRY {}
impl ::core::fmt::Debug for FILE_LAYOUT_INFO_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_LAYOUT_INFO_ENTRY").field("BasicInformation", &self.BasicInformation).field("OwnerId", &self.OwnerId).field("SecurityId", &self.SecurityId).field("Usn", &self.Usn).field("StorageReserveId", &self.StorageReserveId).finish()
    }
}
impl ::core::default::Default for FILE_LAYOUT_INFO_ENTRY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_LAYOUT_INFO_ENTRY_0 {
    fn eq(&self, other: &Self) -> bool {
        self.CreationTime == other.CreationTime && self.LastAccessTime == other.LastAccessTime && self.LastWriteTime == other.LastWriteTime && self.ChangeTime == other.ChangeTime && self.FileAttributes == other.FileAttributes
    }
}
impl ::core::cmp::Eq for FILE_LAYOUT_INFO_ENTRY_0 {}
impl ::core::fmt::Debug for FILE_LAYOUT_INFO_ENTRY_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_LAYOUT_INFO_ENTRY_0").field("CreationTime", &self.CreationTime).field("LastAccessTime", &self.LastAccessTime).field("LastWriteTime", &self.LastWriteTime).field("ChangeTime", &self.ChangeTime).field("FileAttributes", &self.FileAttributes).finish()
    }
}
impl ::core::default::Default for FILE_LAYOUT_NAME_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_LAYOUT_NAME_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.NextNameOffset == other.NextNameOffset && self.Flags == other.Flags && self.ParentFileReferenceNumber == other.ParentFileReferenceNumber && self.FileNameLength == other.FileNameLength && self.Reserved == other.Reserved && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FILE_LAYOUT_NAME_ENTRY {}
impl ::core::fmt::Debug for FILE_LAYOUT_NAME_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_LAYOUT_NAME_ENTRY").field("NextNameOffset", &self.NextNameOffset).field("Flags", &self.Flags).field("ParentFileReferenceNumber", &self.ParentFileReferenceNumber).field("FileNameLength", &self.FileNameLength).field("Reserved", &self.Reserved).field("FileName", &self.FileName).finish()
    }
}
impl ::core::default::Default for FILE_LEVEL_TRIM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_LEVEL_TRIM {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key && self.NumRanges == other.NumRanges && self.Ranges == other.Ranges
    }
}
impl ::core::cmp::Eq for FILE_LEVEL_TRIM {}
impl ::core::fmt::Debug for FILE_LEVEL_TRIM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_LEVEL_TRIM").field("Key", &self.Key).field("NumRanges", &self.NumRanges).field("Ranges", &self.Ranges).finish()
    }
}
impl ::core::default::Default for FILE_LEVEL_TRIM_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_LEVEL_TRIM_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.NumRangesProcessed == other.NumRangesProcessed
    }
}
impl ::core::cmp::Eq for FILE_LEVEL_TRIM_OUTPUT {}
impl ::core::fmt::Debug for FILE_LEVEL_TRIM_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_LEVEL_TRIM_OUTPUT").field("NumRangesProcessed", &self.NumRangesProcessed).finish()
    }
}
impl ::core::default::Default for FILE_LEVEL_TRIM_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_LEVEL_TRIM_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for FILE_LEVEL_TRIM_RANGE {}
impl ::core::fmt::Debug for FILE_LEVEL_TRIM_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_LEVEL_TRIM_RANGE").field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_MAKE_COMPATIBLE_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILE_MAKE_COMPATIBLE_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.CloseDisc == other.CloseDisc
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILE_MAKE_COMPATIBLE_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FILE_MAKE_COMPATIBLE_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_MAKE_COMPATIBLE_BUFFER").field("CloseDisc", &self.CloseDisc).finish()
    }
}
impl ::core::default::Default for FILE_OBJECTID_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FILE_PREFETCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_PREFETCH {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Count == other.Count && self.Prefetch == other.Prefetch
    }
}
impl ::core::cmp::Eq for FILE_PREFETCH {}
impl ::core::fmt::Debug for FILE_PREFETCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_PREFETCH").field("Type", &self.Type).field("Count", &self.Count).field("Prefetch", &self.Prefetch).finish()
    }
}
impl ::core::default::Default for FILE_PREFETCH_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_PREFETCH_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Count == other.Count && self.Context == other.Context && self.Prefetch == other.Prefetch
    }
}
impl ::core::cmp::Eq for FILE_PREFETCH_EX {}
impl ::core::fmt::Debug for FILE_PREFETCH_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_PREFETCH_EX").field("Type", &self.Type).field("Count", &self.Count).field("Context", &self.Context).field("Prefetch", &self.Prefetch).finish()
    }
}
impl ::core::default::Default for FILE_PROVIDER_EXTERNAL_INFO_V0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_PROVIDER_EXTERNAL_INFO_V0 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Algorithm == other.Algorithm
    }
}
impl ::core::cmp::Eq for FILE_PROVIDER_EXTERNAL_INFO_V0 {}
impl ::core::fmt::Debug for FILE_PROVIDER_EXTERNAL_INFO_V0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_PROVIDER_EXTERNAL_INFO_V0").field("Version", &self.Version).field("Algorithm", &self.Algorithm).finish()
    }
}
impl ::core::default::Default for FILE_PROVIDER_EXTERNAL_INFO_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_PROVIDER_EXTERNAL_INFO_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Algorithm == other.Algorithm && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for FILE_PROVIDER_EXTERNAL_INFO_V1 {}
impl ::core::fmt::Debug for FILE_PROVIDER_EXTERNAL_INFO_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_PROVIDER_EXTERNAL_INFO_V1").field("Version", &self.Version).field("Algorithm", &self.Algorithm).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.DirectoryCount == other.DirectoryCount && self.FileCount == other.FileCount && self.FsFormatMajVersion == other.FsFormatMajVersion && self.FsFormatMinVersion == other.FsFormatMinVersion && self.FsFormatName == other.FsFormatName && self.FormatTime == other.FormatTime && self.LastUpdateTime == other.LastUpdateTime && self.CopyrightInfo == other.CopyrightInfo && self.AbstractInfo == other.AbstractInfo && self.FormattingImplementationInfo == other.FormattingImplementationInfo && self.LastModifyingImplementationInfo == other.LastModifyingImplementationInfo
    }
}
impl ::core::cmp::Eq for FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {}
impl ::core::fmt::Debug for FILE_QUERY_ON_DISK_VOL_INFO_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_QUERY_ON_DISK_VOL_INFO_BUFFER")
            .field("DirectoryCount", &self.DirectoryCount)
            .field("FileCount", &self.FileCount)
            .field("FsFormatMajVersion", &self.FsFormatMajVersion)
            .field("FsFormatMinVersion", &self.FsFormatMinVersion)
            .field("FsFormatName", &self.FsFormatName)
            .field("FormatTime", &self.FormatTime)
            .field("LastUpdateTime", &self.LastUpdateTime)
            .field("CopyrightInfo", &self.CopyrightInfo)
            .field("AbstractInfo", &self.AbstractInfo)
            .field("FormattingImplementationInfo", &self.FormattingImplementationInfo)
            .field("LastModifyingImplementationInfo", &self.LastModifyingImplementationInfo)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_QUERY_SPARING_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILE_QUERY_SPARING_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.SparingUnitBytes == other.SparingUnitBytes && self.SoftwareSparing == other.SoftwareSparing && self.TotalSpareBlocks == other.TotalSpareBlocks && self.FreeSpareBlocks == other.FreeSpareBlocks
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILE_QUERY_SPARING_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FILE_QUERY_SPARING_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_QUERY_SPARING_BUFFER").field("SparingUnitBytes", &self.SparingUnitBytes).field("SoftwareSparing", &self.SoftwareSparing).field("TotalSpareBlocks", &self.TotalSpareBlocks).field("FreeSpareBlocks", &self.FreeSpareBlocks).finish()
    }
}
impl ::core::default::Default for FILE_REFERENCE_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_REFERENCE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartingFileReferenceNumber == other.StartingFileReferenceNumber && self.EndingFileReferenceNumber == other.EndingFileReferenceNumber
    }
}
impl ::core::cmp::Eq for FILE_REFERENCE_RANGE {}
impl ::core::fmt::Debug for FILE_REFERENCE_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_REFERENCE_RANGE").field("StartingFileReferenceNumber", &self.StartingFileReferenceNumber).field("EndingFileReferenceNumber", &self.EndingFileReferenceNumber).finish()
    }
}
impl ::core::default::Default for FILE_REGION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_REGION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FileOffset == other.FileOffset && self.Length == other.Length && self.Usage == other.Usage && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for FILE_REGION_INFO {}
impl ::core::fmt::Debug for FILE_REGION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_REGION_INFO").field("FileOffset", &self.FileOffset).field("Length", &self.Length).field("Usage", &self.Usage).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for FILE_REGION_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_REGION_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.FileOffset == other.FileOffset && self.Length == other.Length && self.DesiredUsage == other.DesiredUsage
    }
}
impl ::core::cmp::Eq for FILE_REGION_INPUT {}
impl ::core::fmt::Debug for FILE_REGION_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_REGION_INPUT").field("FileOffset", &self.FileOffset).field("Length", &self.Length).field("DesiredUsage", &self.DesiredUsage).finish()
    }
}
impl ::core::default::Default for FILE_REGION_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_REGION_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.TotalRegionEntryCount == other.TotalRegionEntryCount && self.RegionEntryCount == other.RegionEntryCount && self.Reserved == other.Reserved && self.Region == other.Region
    }
}
impl ::core::cmp::Eq for FILE_REGION_OUTPUT {}
impl ::core::fmt::Debug for FILE_REGION_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_REGION_OUTPUT").field("Flags", &self.Flags).field("TotalRegionEntryCount", &self.TotalRegionEntryCount).field("RegionEntryCount", &self.RegionEntryCount).field("Reserved", &self.Reserved).field("Region", &self.Region).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_SET_DEFECT_MGMT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILE_SET_DEFECT_MGMT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.Disable == other.Disable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILE_SET_DEFECT_MGMT_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FILE_SET_DEFECT_MGMT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_SET_DEFECT_MGMT_BUFFER").field("Disable", &self.Disable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_SET_SPARSE_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILE_SET_SPARSE_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.SetSparse == other.SetSparse
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILE_SET_SPARSE_BUFFER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FILE_SET_SPARSE_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_SET_SPARSE_BUFFER").field("SetSparse", &self.SetSparse).finish()
    }
}
impl ::core::default::Default for FILE_STORAGE_TIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_STORAGE_TIER {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id && self.Name == other.Name && self.Description == other.Description && self.Flags == other.Flags && self.ProvisionedCapacity == other.ProvisionedCapacity && self.MediaType == other.MediaType && self.Class == other.Class
    }
}
impl ::core::cmp::Eq for FILE_STORAGE_TIER {}
impl ::core::fmt::Debug for FILE_STORAGE_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_STORAGE_TIER").field("Id", &self.Id).field("Name", &self.Name).field("Description", &self.Description).field("Flags", &self.Flags).field("ProvisionedCapacity", &self.ProvisionedCapacity).field("MediaType", &self.MediaType).field("Class", &self.Class).finish()
    }
}
impl ::core::default::Default for FILE_STORAGE_TIER_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_STORAGE_TIER_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_STORAGE_TIER_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILE_STORAGE_TIER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_STORAGE_TIER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_STORAGE_TIER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FILE_STORAGE_TIER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FILE_STORAGE_TIER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FILE_STORAGE_TIER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FILE_STORAGE_TIER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FILE_STORAGE_TIER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for FILE_STORAGE_TIER_MEDIA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FILE_STORAGE_TIER_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILE_STORAGE_TIER_MEDIA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FILE_STORAGE_TIER_REGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_STORAGE_TIER_REGION {
    fn eq(&self, other: &Self) -> bool {
        self.TierId == other.TierId && self.Offset == other.Offset && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for FILE_STORAGE_TIER_REGION {}
impl ::core::fmt::Debug for FILE_STORAGE_TIER_REGION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_STORAGE_TIER_REGION").field("TierId", &self.TierId).field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FILE_SYSTEM_RECOGNITION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FILE_SYSTEM_RECOGNITION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.FileSystem == other.FileSystem
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FILE_SYSTEM_RECOGNITION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FILE_SYSTEM_RECOGNITION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_SYSTEM_RECOGNITION_INFORMATION").field("FileSystem", &self.FileSystem).finish()
    }
}
impl ::core::default::Default for FILE_TYPE_NOTIFICATION_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_TYPE_NOTIFICATION_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.NumFileTypeIDs == other.NumFileTypeIDs && self.FileTypeID == other.FileTypeID
    }
}
impl ::core::cmp::Eq for FILE_TYPE_NOTIFICATION_INPUT {}
impl ::core::fmt::Debug for FILE_TYPE_NOTIFICATION_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_TYPE_NOTIFICATION_INPUT").field("Flags", &self.Flags).field("NumFileTypeIDs", &self.NumFileTypeIDs).field("FileTypeID", &self.FileTypeID).finish()
    }
}
impl ::core::default::Default for FILE_ZERO_DATA_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_ZERO_DATA_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.FileOffset == other.FileOffset && self.BeyondFinalZero == other.BeyondFinalZero
    }
}
impl ::core::cmp::Eq for FILE_ZERO_DATA_INFORMATION {}
impl ::core::fmt::Debug for FILE_ZERO_DATA_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ZERO_DATA_INFORMATION").field("FileOffset", &self.FileOffset).field("BeyondFinalZero", &self.BeyondFinalZero).finish()
    }
}
impl ::core::default::Default for FILE_ZERO_DATA_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FILE_ZERO_DATA_INFORMATION_EX {
    fn eq(&self, other: &Self) -> bool {
        self.FileOffset == other.FileOffset && self.BeyondFinalZero == other.BeyondFinalZero && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for FILE_ZERO_DATA_INFORMATION_EX {}
impl ::core::fmt::Debug for FILE_ZERO_DATA_INFORMATION_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FILE_ZERO_DATA_INFORMATION_EX").field("FileOffset", &self.FileOffset).field("BeyondFinalZero", &self.BeyondFinalZero).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::default::Default for FIND_BY_SID_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::PartialEq for FIND_BY_SID_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Restart == other.Restart && self.Sid == other.Sid
    }
}
#[cfg(feature = "Win32_Security")]
impl ::core::cmp::Eq for FIND_BY_SID_DATA {}
#[cfg(feature = "Win32_Security")]
impl ::core::fmt::Debug for FIND_BY_SID_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FIND_BY_SID_DATA").field("Restart", &self.Restart).field("Sid", &self.Sid).finish()
    }
}
impl ::core::default::Default for FIND_BY_SID_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FIND_BY_SID_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.FileIndex == other.FileIndex && self.FileNameLength == other.FileNameLength && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for FIND_BY_SID_OUTPUT {}
impl ::core::fmt::Debug for FIND_BY_SID_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FIND_BY_SID_OUTPUT").field("NextEntryOffset", &self.NextEntryOffset).field("FileIndex", &self.FileIndex).field("FileNameLength", &self.FileNameLength).field("FileName", &self.FileName).finish()
    }
}
impl ::core::default::Default for FORMAT_EX_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FORMAT_EX_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.MediaType == other.MediaType && self.StartCylinderNumber == other.StartCylinderNumber && self.EndCylinderNumber == other.EndCylinderNumber && self.StartHeadNumber == other.StartHeadNumber && self.EndHeadNumber == other.EndHeadNumber && self.FormatGapLength == other.FormatGapLength && self.SectorsPerTrack == other.SectorsPerTrack && self.SectorNumber == other.SectorNumber
    }
}
impl ::core::cmp::Eq for FORMAT_EX_PARAMETERS {}
impl ::core::fmt::Debug for FORMAT_EX_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FORMAT_EX_PARAMETERS").field("MediaType", &self.MediaType).field("StartCylinderNumber", &self.StartCylinderNumber).field("EndCylinderNumber", &self.EndCylinderNumber).field("StartHeadNumber", &self.StartHeadNumber).field("EndHeadNumber", &self.EndHeadNumber).field("FormatGapLength", &self.FormatGapLength).field("SectorsPerTrack", &self.SectorsPerTrack).field("SectorNumber", &self.SectorNumber).finish()
    }
}
impl ::core::default::Default for FORMAT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FORMAT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.MediaType == other.MediaType && self.StartCylinderNumber == other.StartCylinderNumber && self.EndCylinderNumber == other.EndCylinderNumber && self.StartHeadNumber == other.StartHeadNumber && self.EndHeadNumber == other.EndHeadNumber
    }
}
impl ::core::cmp::Eq for FORMAT_PARAMETERS {}
impl ::core::fmt::Debug for FORMAT_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FORMAT_PARAMETERS").field("MediaType", &self.MediaType).field("StartCylinderNumber", &self.StartCylinderNumber).field("EndCylinderNumber", &self.EndCylinderNumber).field("StartHeadNumber", &self.StartHeadNumber).field("EndHeadNumber", &self.EndHeadNumber).finish()
    }
}
impl ::core::default::Default for FSCTL_GET_INTEGRITY_INFORMATION_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FSCTL_GET_INTEGRITY_INFORMATION_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.ChecksumAlgorithm == other.ChecksumAlgorithm && self.Reserved == other.Reserved && self.Flags == other.Flags && self.ChecksumChunkSizeInBytes == other.ChecksumChunkSizeInBytes && self.ClusterSizeInBytes == other.ClusterSizeInBytes
    }
}
impl ::core::cmp::Eq for FSCTL_GET_INTEGRITY_INFORMATION_BUFFER {}
impl ::core::fmt::Debug for FSCTL_GET_INTEGRITY_INFORMATION_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_GET_INTEGRITY_INFORMATION_BUFFER").field("ChecksumAlgorithm", &self.ChecksumAlgorithm).field("Reserved", &self.Reserved).field("Flags", &self.Flags).field("ChecksumChunkSizeInBytes", &self.ChecksumChunkSizeInBytes).field("ClusterSizeInBytes", &self.ClusterSizeInBytes).finish()
    }
}
impl ::core::default::Default for FSCTL_OFFLOAD_READ_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FSCTL_OFFLOAD_READ_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Flags == other.Flags && self.TokenTimeToLive == other.TokenTimeToLive && self.Reserved == other.Reserved && self.FileOffset == other.FileOffset && self.CopyLength == other.CopyLength
    }
}
impl ::core::cmp::Eq for FSCTL_OFFLOAD_READ_INPUT {}
impl ::core::fmt::Debug for FSCTL_OFFLOAD_READ_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_OFFLOAD_READ_INPUT").field("Size", &self.Size).field("Flags", &self.Flags).field("TokenTimeToLive", &self.TokenTimeToLive).field("Reserved", &self.Reserved).field("FileOffset", &self.FileOffset).field("CopyLength", &self.CopyLength).finish()
    }
}
impl ::core::default::Default for FSCTL_OFFLOAD_READ_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FSCTL_OFFLOAD_READ_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Flags == other.Flags && self.TransferLength == other.TransferLength && self.Token == other.Token
    }
}
impl ::core::cmp::Eq for FSCTL_OFFLOAD_READ_OUTPUT {}
impl ::core::fmt::Debug for FSCTL_OFFLOAD_READ_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_OFFLOAD_READ_OUTPUT").field("Size", &self.Size).field("Flags", &self.Flags).field("TransferLength", &self.TransferLength).field("Token", &self.Token).finish()
    }
}
impl ::core::default::Default for FSCTL_OFFLOAD_WRITE_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FSCTL_OFFLOAD_WRITE_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Flags == other.Flags && self.FileOffset == other.FileOffset && self.CopyLength == other.CopyLength && self.TransferOffset == other.TransferOffset && self.Token == other.Token
    }
}
impl ::core::cmp::Eq for FSCTL_OFFLOAD_WRITE_INPUT {}
impl ::core::fmt::Debug for FSCTL_OFFLOAD_WRITE_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_OFFLOAD_WRITE_INPUT").field("Size", &self.Size).field("Flags", &self.Flags).field("FileOffset", &self.FileOffset).field("CopyLength", &self.CopyLength).field("TransferOffset", &self.TransferOffset).field("Token", &self.Token).finish()
    }
}
impl ::core::default::Default for FSCTL_OFFLOAD_WRITE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FSCTL_OFFLOAD_WRITE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Flags == other.Flags && self.LengthWritten == other.LengthWritten
    }
}
impl ::core::cmp::Eq for FSCTL_OFFLOAD_WRITE_OUTPUT {}
impl ::core::fmt::Debug for FSCTL_OFFLOAD_WRITE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_OFFLOAD_WRITE_OUTPUT").field("Size", &self.Size).field("Flags", &self.Flags).field("LengthWritten", &self.LengthWritten).finish()
    }
}
impl ::core::default::Default for FSCTL_QUERY_FAT_BPB_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FSCTL_QUERY_FAT_BPB_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.First0x24BytesOfBootSector == other.First0x24BytesOfBootSector
    }
}
impl ::core::cmp::Eq for FSCTL_QUERY_FAT_BPB_BUFFER {}
impl ::core::fmt::Debug for FSCTL_QUERY_FAT_BPB_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_QUERY_FAT_BPB_BUFFER").field("First0x24BytesOfBootSector", &self.First0x24BytesOfBootSector).finish()
    }
}
impl ::core::default::Default for FSCTL_QUERY_REGION_INFO_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FSCTL_QUERY_REGION_INFO_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Flags == other.Flags && self.NumberOfTierIds == other.NumberOfTierIds && self.TierIds == other.TierIds
    }
}
impl ::core::cmp::Eq for FSCTL_QUERY_REGION_INFO_INPUT {}
impl ::core::fmt::Debug for FSCTL_QUERY_REGION_INFO_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_QUERY_REGION_INFO_INPUT").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("NumberOfTierIds", &self.NumberOfTierIds).field("TierIds", &self.TierIds).finish()
    }
}
impl ::core::default::Default for FSCTL_QUERY_REGION_INFO_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FSCTL_QUERY_REGION_INFO_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Flags == other.Flags && self.Reserved == other.Reserved && self.Alignment == other.Alignment && self.TotalNumberOfRegions == other.TotalNumberOfRegions && self.NumberOfRegionsReturned == other.NumberOfRegionsReturned && self.Regions == other.Regions
    }
}
impl ::core::cmp::Eq for FSCTL_QUERY_REGION_INFO_OUTPUT {}
impl ::core::fmt::Debug for FSCTL_QUERY_REGION_INFO_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_QUERY_REGION_INFO_OUTPUT").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Alignment", &self.Alignment).field("TotalNumberOfRegions", &self.TotalNumberOfRegions).field("NumberOfRegionsReturned", &self.NumberOfRegionsReturned).field("Regions", &self.Regions).finish()
    }
}
impl ::core::default::Default for FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Flags == other.Flags && self.TotalNumberOfTiers == other.TotalNumberOfTiers && self.NumberOfTiersReturned == other.NumberOfTiersReturned && self.Tiers == other.Tiers
    }
}
impl ::core::cmp::Eq for FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {}
impl ::core::fmt::Debug for FSCTL_QUERY_STORAGE_CLASSES_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_QUERY_STORAGE_CLASSES_OUTPUT").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("TotalNumberOfTiers", &self.TotalNumberOfTiers).field("NumberOfTiersReturned", &self.NumberOfTiersReturned).field("Tiers", &self.Tiers).finish()
    }
}
impl ::core::default::Default for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.ChecksumAlgorithm == other.ChecksumAlgorithm && self.Reserved == other.Reserved && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER {}
impl ::core::fmt::Debug for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_SET_INTEGRITY_INFORMATION_BUFFER").field("ChecksumAlgorithm", &self.ChecksumAlgorithm).field("Reserved", &self.Reserved).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {
    fn eq(&self, other: &Self) -> bool {
        self.EnableIntegrity == other.EnableIntegrity && self.KeepIntegrityStateUnchanged == other.KeepIntegrityStateUnchanged && self.Reserved == other.Reserved && self.Flags == other.Flags && self.Version == other.Version && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {}
impl ::core::fmt::Debug for FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FSCTL_SET_INTEGRITY_INFORMATION_BUFFER_EX").field("EnableIntegrity", &self.EnableIntegrity).field("KeepIntegrityStateUnchanged", &self.KeepIntegrityStateUnchanged).field("Reserved", &self.Reserved).field("Flags", &self.Flags).field("Version", &self.Version).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::default::Default for FS_BPIO_INFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FS_BPIO_INFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FS_BPIO_INFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FS_BPIO_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FS_BPIO_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ActiveBypassIoCount == other.ActiveBypassIoCount && self.StorageDriverNameLen == other.StorageDriverNameLen && self.StorageDriverName == other.StorageDriverName
    }
}
impl ::core::cmp::Eq for FS_BPIO_INFO {}
impl ::core::fmt::Debug for FS_BPIO_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FS_BPIO_INFO").field("ActiveBypassIoCount", &self.ActiveBypassIoCount).field("StorageDriverNameLen", &self.StorageDriverNameLen).field("StorageDriverName", &self.StorageDriverName).finish()
    }
}
impl ::core::default::Default for FS_BPIO_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FS_BPIO_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Operation == other.Operation && self.InFlags == other.InFlags && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for FS_BPIO_INPUT {}
impl ::core::fmt::Debug for FS_BPIO_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FS_BPIO_INPUT").field("Operation", &self.Operation).field("InFlags", &self.InFlags).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::default::Default for FS_BPIO_OPERATIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FS_BPIO_OPERATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FS_BPIO_OPERATIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FS_BPIO_OUTFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FS_BPIO_OUTFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FS_BPIO_OUTFLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FS_BPIO_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FS_BPIO_RESULTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FS_BPIO_RESULTS {
    fn eq(&self, other: &Self) -> bool {
        self.OpStatus == other.OpStatus && self.FailingDriverNameLen == other.FailingDriverNameLen && self.FailingDriverName == other.FailingDriverName && self.FailureReasonLen == other.FailureReasonLen && self.FailureReason == other.FailureReason
    }
}
impl ::core::cmp::Eq for FS_BPIO_RESULTS {}
impl ::core::fmt::Debug for FS_BPIO_RESULTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FS_BPIO_RESULTS").field("OpStatus", &self.OpStatus).field("FailingDriverNameLen", &self.FailingDriverNameLen).field("FailingDriverName", &self.FailingDriverName).field("FailureReasonLen", &self.FailureReasonLen).field("FailureReason", &self.FailureReason).finish()
    }
}
impl ::core::default::Default for GETVERSIONINPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for GET_CHANGER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GET_CHANGER_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.NumberTransportElements == other.NumberTransportElements
            && self.NumberStorageElements == other.NumberStorageElements
            && self.NumberCleanerSlots == other.NumberCleanerSlots
            && self.NumberIEElements == other.NumberIEElements
            && self.NumberDataTransferElements == other.NumberDataTransferElements
            && self.NumberOfDoors == other.NumberOfDoors
            && self.FirstSlotNumber == other.FirstSlotNumber
            && self.FirstDriveNumber == other.FirstDriveNumber
            && self.FirstTransportNumber == other.FirstTransportNumber
            && self.FirstIEPortNumber == other.FirstIEPortNumber
            && self.FirstCleanerSlotAddress == other.FirstCleanerSlotAddress
            && self.MagazineSize == other.MagazineSize
            && self.DriveCleanTimeout == other.DriveCleanTimeout
            && self.Features0 == other.Features0
            && self.Features1 == other.Features1
            && self.MoveFromTransport == other.MoveFromTransport
            && self.MoveFromSlot == other.MoveFromSlot
            && self.MoveFromIePort == other.MoveFromIePort
            && self.MoveFromDrive == other.MoveFromDrive
            && self.ExchangeFromTransport == other.ExchangeFromTransport
            && self.ExchangeFromSlot == other.ExchangeFromSlot
            && self.ExchangeFromIePort == other.ExchangeFromIePort
            && self.ExchangeFromDrive == other.ExchangeFromDrive
            && self.LockUnlockCapabilities == other.LockUnlockCapabilities
            && self.PositionCapabilities == other.PositionCapabilities
            && self.Reserved1 == other.Reserved1
            && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for GET_CHANGER_PARAMETERS {}
impl ::core::fmt::Debug for GET_CHANGER_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_CHANGER_PARAMETERS")
            .field("Size", &self.Size)
            .field("NumberTransportElements", &self.NumberTransportElements)
            .field("NumberStorageElements", &self.NumberStorageElements)
            .field("NumberCleanerSlots", &self.NumberCleanerSlots)
            .field("NumberIEElements", &self.NumberIEElements)
            .field("NumberDataTransferElements", &self.NumberDataTransferElements)
            .field("NumberOfDoors", &self.NumberOfDoors)
            .field("FirstSlotNumber", &self.FirstSlotNumber)
            .field("FirstDriveNumber", &self.FirstDriveNumber)
            .field("FirstTransportNumber", &self.FirstTransportNumber)
            .field("FirstIEPortNumber", &self.FirstIEPortNumber)
            .field("FirstCleanerSlotAddress", &self.FirstCleanerSlotAddress)
            .field("MagazineSize", &self.MagazineSize)
            .field("DriveCleanTimeout", &self.DriveCleanTimeout)
            .field("Features0", &self.Features0)
            .field("Features1", &self.Features1)
            .field("MoveFromTransport", &self.MoveFromTransport)
            .field("MoveFromSlot", &self.MoveFromSlot)
            .field("MoveFromIePort", &self.MoveFromIePort)
            .field("MoveFromDrive", &self.MoveFromDrive)
            .field("ExchangeFromTransport", &self.ExchangeFromTransport)
            .field("ExchangeFromSlot", &self.ExchangeFromSlot)
            .field("ExchangeFromIePort", &self.ExchangeFromIePort)
            .field("ExchangeFromDrive", &self.ExchangeFromDrive)
            .field("LockUnlockCapabilities", &self.LockUnlockCapabilities)
            .field("PositionCapabilities", &self.PositionCapabilities)
            .field("Reserved1", &self.Reserved1)
            .field("Reserved2", &self.Reserved2)
            .finish()
    }
}
impl ::core::default::Default for GET_CHANGER_PARAMETERS_FEATURES1 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GET_CHANGER_PARAMETERS_FEATURES1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_CHANGER_PARAMETERS_FEATURES1").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GET_CHANGER_PARAMETERS_FEATURES1 {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GET_CHANGER_PARAMETERS_FEATURES1 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GET_CHANGER_PARAMETERS_FEATURES1 {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GET_CHANGER_PARAMETERS_FEATURES1 {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GET_CHANGER_PARAMETERS_FEATURES1 {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.RequestDataType == other.RequestDataType && self.RequestDataSet == other.RequestDataSet
    }
}
impl ::core::cmp::Eq for GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST {}
impl ::core::fmt::Debug for GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_DEVICE_INTERNAL_STATUS_DATA_REQUEST").field("Version", &self.Version).field("Size", &self.Size).field("RequestDataType", &self.RequestDataType).field("RequestDataSet", &self.RequestDataSet).finish()
    }
}
impl ::core::default::Default for GET_DISK_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GET_DISK_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Reserved1 == other.Reserved1 && self.Attributes == other.Attributes
    }
}
impl ::core::cmp::Eq for GET_DISK_ATTRIBUTES {}
impl ::core::fmt::Debug for GET_DISK_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_DISK_ATTRIBUTES").field("Version", &self.Version).field("Reserved1", &self.Reserved1).field("Attributes", &self.Attributes).finish()
    }
}
impl ::core::default::Default for GET_FILTER_FILE_IDENTIFIER_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GET_FILTER_FILE_IDENTIFIER_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.AltitudeLength == other.AltitudeLength && self.Altitude == other.Altitude
    }
}
impl ::core::cmp::Eq for GET_FILTER_FILE_IDENTIFIER_INPUT {}
impl ::core::fmt::Debug for GET_FILTER_FILE_IDENTIFIER_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_FILTER_FILE_IDENTIFIER_INPUT").field("AltitudeLength", &self.AltitudeLength).field("Altitude", &self.Altitude).finish()
    }
}
impl ::core::default::Default for GET_FILTER_FILE_IDENTIFIER_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GET_FILTER_FILE_IDENTIFIER_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.FilterFileIdentifierLength == other.FilterFileIdentifierLength && self.FilterFileIdentifier == other.FilterFileIdentifier
    }
}
impl ::core::cmp::Eq for GET_FILTER_FILE_IDENTIFIER_OUTPUT {}
impl ::core::fmt::Debug for GET_FILTER_FILE_IDENTIFIER_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_FILTER_FILE_IDENTIFIER_OUTPUT").field("FilterFileIdentifierLength", &self.FilterFileIdentifierLength).field("FilterFileIdentifier", &self.FilterFileIdentifier).finish()
    }
}
impl ::core::default::Default for GET_LENGTH_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for GET_LENGTH_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
    }
}
impl ::core::cmp::Eq for GET_LENGTH_INFORMATION {}
impl ::core::fmt::Debug for GET_LENGTH_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GET_LENGTH_INFORMATION").field("Length", &self.Length).finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for GET_MEDIA_TYPES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for GPT_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for GPT_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GPT_ATTRIBUTES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for GPT_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for GPT_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for GPT_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for GPT_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for GPT_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for GP_LOG_PAGE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for HISTOGRAM_BUCKET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for HISTOGRAM_BUCKET {
    fn eq(&self, other: &Self) -> bool {
        self.Reads == other.Reads && self.Writes == other.Writes
    }
}
impl ::core::cmp::Eq for HISTOGRAM_BUCKET {}
impl ::core::fmt::Debug for HISTOGRAM_BUCKET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HISTOGRAM_BUCKET").field("Reads", &self.Reads).field("Writes", &self.Writes).finish()
    }
}
impl ::core::default::Default for IDEREGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IDEREGS {
    fn eq(&self, other: &Self) -> bool {
        self.bFeaturesReg == other.bFeaturesReg && self.bSectorCountReg == other.bSectorCountReg && self.bSectorNumberReg == other.bSectorNumberReg && self.bCylLowReg == other.bCylLowReg && self.bCylHighReg == other.bCylHighReg && self.bDriveHeadReg == other.bDriveHeadReg && self.bCommandReg == other.bCommandReg && self.bReserved == other.bReserved
    }
}
impl ::core::cmp::Eq for IDEREGS {}
impl ::core::fmt::Debug for IDEREGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IDEREGS").field("bFeaturesReg", &self.bFeaturesReg).field("bSectorCountReg", &self.bSectorCountReg).field("bSectorNumberReg", &self.bSectorNumberReg).field("bCylLowReg", &self.bCylLowReg).field("bCylHighReg", &self.bCylHighReg).field("bDriveHeadReg", &self.bDriveHeadReg).field("bCommandReg", &self.bCommandReg).field("bReserved", &self.bReserved).finish()
    }
}
impl ::core::default::Default for IO_IRP_EXT_TRACK_OFFSET_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for LOOKUP_STREAM_FROM_CLUSTER_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LOOKUP_STREAM_FROM_CLUSTER_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.OffsetToNext == other.OffsetToNext && self.Flags == other.Flags && self.Reserved == other.Reserved && self.Cluster == other.Cluster && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for LOOKUP_STREAM_FROM_CLUSTER_ENTRY {}
impl ::core::fmt::Debug for LOOKUP_STREAM_FROM_CLUSTER_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOOKUP_STREAM_FROM_CLUSTER_ENTRY").field("OffsetToNext", &self.OffsetToNext).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Cluster", &self.Cluster).field("FileName", &self.FileName).finish()
    }
}
impl ::core::default::Default for LOOKUP_STREAM_FROM_CLUSTER_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LOOKUP_STREAM_FROM_CLUSTER_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.NumberOfClusters == other.NumberOfClusters && self.Cluster == other.Cluster
    }
}
impl ::core::cmp::Eq for LOOKUP_STREAM_FROM_CLUSTER_INPUT {}
impl ::core::fmt::Debug for LOOKUP_STREAM_FROM_CLUSTER_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOOKUP_STREAM_FROM_CLUSTER_INPUT").field("Flags", &self.Flags).field("NumberOfClusters", &self.NumberOfClusters).field("Cluster", &self.Cluster).finish()
    }
}
impl ::core::default::Default for LOOKUP_STREAM_FROM_CLUSTER_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for LOOKUP_STREAM_FROM_CLUSTER_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.NumberOfMatches == other.NumberOfMatches && self.BufferSizeRequired == other.BufferSizeRequired
    }
}
impl ::core::cmp::Eq for LOOKUP_STREAM_FROM_CLUSTER_OUTPUT {}
impl ::core::fmt::Debug for LOOKUP_STREAM_FROM_CLUSTER_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOOKUP_STREAM_FROM_CLUSTER_OUTPUT").field("Offset", &self.Offset).field("NumberOfMatches", &self.NumberOfMatches).field("BufferSizeRequired", &self.BufferSizeRequired).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MARK_HANDLE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for MARK_HANDLE_INFO32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MEDIA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEDIA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for MFT_ENUM_DATA_V0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFT_ENUM_DATA_V0 {
    fn eq(&self, other: &Self) -> bool {
        self.StartFileReferenceNumber == other.StartFileReferenceNumber && self.LowUsn == other.LowUsn && self.HighUsn == other.HighUsn
    }
}
impl ::core::cmp::Eq for MFT_ENUM_DATA_V0 {}
impl ::core::fmt::Debug for MFT_ENUM_DATA_V0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFT_ENUM_DATA_V0").field("StartFileReferenceNumber", &self.StartFileReferenceNumber).field("LowUsn", &self.LowUsn).field("HighUsn", &self.HighUsn).finish()
    }
}
impl ::core::default::Default for MFT_ENUM_DATA_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MFT_ENUM_DATA_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.StartFileReferenceNumber == other.StartFileReferenceNumber && self.LowUsn == other.LowUsn && self.HighUsn == other.HighUsn && self.MinMajorVersion == other.MinMajorVersion && self.MaxMajorVersion == other.MaxMajorVersion
    }
}
impl ::core::cmp::Eq for MFT_ENUM_DATA_V1 {}
impl ::core::fmt::Debug for MFT_ENUM_DATA_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MFT_ENUM_DATA_V1").field("StartFileReferenceNumber", &self.StartFileReferenceNumber).field("LowUsn", &self.LowUsn).field("HighUsn", &self.HighUsn).field("MinMajorVersion", &self.MinMajorVersion).field("MaxMajorVersion", &self.MaxMajorVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MOVE_FILE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MOVE_FILE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.FileHandle == other.FileHandle && self.StartingVcn == other.StartingVcn && self.StartingLcn == other.StartingLcn && self.ClusterCount == other.ClusterCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MOVE_FILE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MOVE_FILE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOVE_FILE_DATA").field("FileHandle", &self.FileHandle).field("StartingVcn", &self.StartingVcn).field("StartingLcn", &self.StartingLcn).field("ClusterCount", &self.ClusterCount).finish()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for MOVE_FILE_DATA32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::PartialEq for MOVE_FILE_DATA32 {
    fn eq(&self, other: &Self) -> bool {
        self.FileHandle == other.FileHandle && self.StartingVcn == other.StartingVcn && self.StartingLcn == other.StartingLcn && self.ClusterCount == other.ClusterCount
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::cmp::Eq for MOVE_FILE_DATA32 {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::fmt::Debug for MOVE_FILE_DATA32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOVE_FILE_DATA32").field("FileHandle", &self.FileHandle).field("StartingVcn", &self.StartingVcn).field("StartingLcn", &self.StartingLcn).field("ClusterCount", &self.ClusterCount).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MOVE_FILE_RECORD_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MOVE_FILE_RECORD_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.FileHandle == other.FileHandle && self.SourceFileRecord == other.SourceFileRecord && self.TargetFileRecord == other.TargetFileRecord
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MOVE_FILE_RECORD_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MOVE_FILE_RECORD_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MOVE_FILE_RECORD_DATA").field("FileHandle", &self.FileHandle).field("SourceFileRecord", &self.SourceFileRecord).field("TargetFileRecord", &self.TargetFileRecord).finish()
    }
}
impl ::core::default::Default for NTFS_EXTENDED_VOLUME_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTFS_EXTENDED_VOLUME_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ByteCount == other.ByteCount && self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.BytesPerPhysicalSector == other.BytesPerPhysicalSector && self.LfsMajorVersion == other.LfsMajorVersion && self.LfsMinorVersion == other.LfsMinorVersion && self.MaxDeviceTrimExtentCount == other.MaxDeviceTrimExtentCount && self.MaxDeviceTrimByteCount == other.MaxDeviceTrimByteCount && self.MaxVolumeTrimExtentCount == other.MaxVolumeTrimExtentCount && self.MaxVolumeTrimByteCount == other.MaxVolumeTrimByteCount
    }
}
impl ::core::cmp::Eq for NTFS_EXTENDED_VOLUME_DATA {}
impl ::core::fmt::Debug for NTFS_EXTENDED_VOLUME_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_EXTENDED_VOLUME_DATA")
            .field("ByteCount", &self.ByteCount)
            .field("MajorVersion", &self.MajorVersion)
            .field("MinorVersion", &self.MinorVersion)
            .field("BytesPerPhysicalSector", &self.BytesPerPhysicalSector)
            .field("LfsMajorVersion", &self.LfsMajorVersion)
            .field("LfsMinorVersion", &self.LfsMinorVersion)
            .field("MaxDeviceTrimExtentCount", &self.MaxDeviceTrimExtentCount)
            .field("MaxDeviceTrimByteCount", &self.MaxDeviceTrimByteCount)
            .field("MaxVolumeTrimExtentCount", &self.MaxVolumeTrimExtentCount)
            .field("MaxVolumeTrimByteCount", &self.MaxVolumeTrimByteCount)
            .finish()
    }
}
impl ::core::default::Default for NTFS_FILE_RECORD_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTFS_FILE_RECORD_INPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.FileReferenceNumber == other.FileReferenceNumber
    }
}
impl ::core::cmp::Eq for NTFS_FILE_RECORD_INPUT_BUFFER {}
impl ::core::fmt::Debug for NTFS_FILE_RECORD_INPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_FILE_RECORD_INPUT_BUFFER").field("FileReferenceNumber", &self.FileReferenceNumber).finish()
    }
}
impl ::core::default::Default for NTFS_FILE_RECORD_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTFS_FILE_RECORD_OUTPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.FileReferenceNumber == other.FileReferenceNumber && self.FileRecordLength == other.FileRecordLength && self.FileRecordBuffer == other.FileRecordBuffer
    }
}
impl ::core::cmp::Eq for NTFS_FILE_RECORD_OUTPUT_BUFFER {}
impl ::core::fmt::Debug for NTFS_FILE_RECORD_OUTPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_FILE_RECORD_OUTPUT_BUFFER").field("FileReferenceNumber", &self.FileReferenceNumber).field("FileRecordLength", &self.FileRecordLength).field("FileRecordBuffer", &self.FileRecordBuffer).finish()
    }
}
impl ::core::default::Default for NTFS_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.LogFileFullExceptions == other.LogFileFullExceptions
            && self.OtherExceptions == other.OtherExceptions
            && self.MftReads == other.MftReads
            && self.MftReadBytes == other.MftReadBytes
            && self.MftWrites == other.MftWrites
            && self.MftWriteBytes == other.MftWriteBytes
            && self.MftWritesUserLevel == other.MftWritesUserLevel
            && self.MftWritesFlushForLogFileFull == other.MftWritesFlushForLogFileFull
            && self.MftWritesLazyWriter == other.MftWritesLazyWriter
            && self.MftWritesUserRequest == other.MftWritesUserRequest
            && self.Mft2Writes == other.Mft2Writes
            && self.Mft2WriteBytes == other.Mft2WriteBytes
            && self.Mft2WritesUserLevel == other.Mft2WritesUserLevel
            && self.Mft2WritesFlushForLogFileFull == other.Mft2WritesFlushForLogFileFull
            && self.Mft2WritesLazyWriter == other.Mft2WritesLazyWriter
            && self.Mft2WritesUserRequest == other.Mft2WritesUserRequest
            && self.RootIndexReads == other.RootIndexReads
            && self.RootIndexReadBytes == other.RootIndexReadBytes
            && self.RootIndexWrites == other.RootIndexWrites
            && self.RootIndexWriteBytes == other.RootIndexWriteBytes
            && self.BitmapReads == other.BitmapReads
            && self.BitmapReadBytes == other.BitmapReadBytes
            && self.BitmapWrites == other.BitmapWrites
            && self.BitmapWriteBytes == other.BitmapWriteBytes
            && self.BitmapWritesFlushForLogFileFull == other.BitmapWritesFlushForLogFileFull
            && self.BitmapWritesLazyWriter == other.BitmapWritesLazyWriter
            && self.BitmapWritesUserRequest == other.BitmapWritesUserRequest
            && self.BitmapWritesUserLevel == other.BitmapWritesUserLevel
            && self.MftBitmapReads == other.MftBitmapReads
            && self.MftBitmapReadBytes == other.MftBitmapReadBytes
            && self.MftBitmapWrites == other.MftBitmapWrites
            && self.MftBitmapWriteBytes == other.MftBitmapWriteBytes
            && self.MftBitmapWritesFlushForLogFileFull == other.MftBitmapWritesFlushForLogFileFull
            && self.MftBitmapWritesLazyWriter == other.MftBitmapWritesLazyWriter
            && self.MftBitmapWritesUserRequest == other.MftBitmapWritesUserRequest
            && self.MftBitmapWritesUserLevel == other.MftBitmapWritesUserLevel
            && self.UserIndexReads == other.UserIndexReads
            && self.UserIndexReadBytes == other.UserIndexReadBytes
            && self.UserIndexWrites == other.UserIndexWrites
            && self.UserIndexWriteBytes == other.UserIndexWriteBytes
            && self.LogFileReads == other.LogFileReads
            && self.LogFileReadBytes == other.LogFileReadBytes
            && self.LogFileWrites == other.LogFileWrites
            && self.LogFileWriteBytes == other.LogFileWriteBytes
            && self.Allocate == other.Allocate
            && self.DiskResourcesExhausted == other.DiskResourcesExhausted
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS {}
impl ::core::fmt::Debug for NTFS_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS")
            .field("LogFileFullExceptions", &self.LogFileFullExceptions)
            .field("OtherExceptions", &self.OtherExceptions)
            .field("MftReads", &self.MftReads)
            .field("MftReadBytes", &self.MftReadBytes)
            .field("MftWrites", &self.MftWrites)
            .field("MftWriteBytes", &self.MftWriteBytes)
            .field("MftWritesUserLevel", &self.MftWritesUserLevel)
            .field("MftWritesFlushForLogFileFull", &self.MftWritesFlushForLogFileFull)
            .field("MftWritesLazyWriter", &self.MftWritesLazyWriter)
            .field("MftWritesUserRequest", &self.MftWritesUserRequest)
            .field("Mft2Writes", &self.Mft2Writes)
            .field("Mft2WriteBytes", &self.Mft2WriteBytes)
            .field("Mft2WritesUserLevel", &self.Mft2WritesUserLevel)
            .field("Mft2WritesFlushForLogFileFull", &self.Mft2WritesFlushForLogFileFull)
            .field("Mft2WritesLazyWriter", &self.Mft2WritesLazyWriter)
            .field("Mft2WritesUserRequest", &self.Mft2WritesUserRequest)
            .field("RootIndexReads", &self.RootIndexReads)
            .field("RootIndexReadBytes", &self.RootIndexReadBytes)
            .field("RootIndexWrites", &self.RootIndexWrites)
            .field("RootIndexWriteBytes", &self.RootIndexWriteBytes)
            .field("BitmapReads", &self.BitmapReads)
            .field("BitmapReadBytes", &self.BitmapReadBytes)
            .field("BitmapWrites", &self.BitmapWrites)
            .field("BitmapWriteBytes", &self.BitmapWriteBytes)
            .field("BitmapWritesFlushForLogFileFull", &self.BitmapWritesFlushForLogFileFull)
            .field("BitmapWritesLazyWriter", &self.BitmapWritesLazyWriter)
            .field("BitmapWritesUserRequest", &self.BitmapWritesUserRequest)
            .field("BitmapWritesUserLevel", &self.BitmapWritesUserLevel)
            .field("MftBitmapReads", &self.MftBitmapReads)
            .field("MftBitmapReadBytes", &self.MftBitmapReadBytes)
            .field("MftBitmapWrites", &self.MftBitmapWrites)
            .field("MftBitmapWriteBytes", &self.MftBitmapWriteBytes)
            .field("MftBitmapWritesFlushForLogFileFull", &self.MftBitmapWritesFlushForLogFileFull)
            .field("MftBitmapWritesLazyWriter", &self.MftBitmapWritesLazyWriter)
            .field("MftBitmapWritesUserRequest", &self.MftBitmapWritesUserRequest)
            .field("MftBitmapWritesUserLevel", &self.MftBitmapWritesUserLevel)
            .field("UserIndexReads", &self.UserIndexReads)
            .field("UserIndexReadBytes", &self.UserIndexReadBytes)
            .field("UserIndexWrites", &self.UserIndexWrites)
            .field("UserIndexWriteBytes", &self.UserIndexWriteBytes)
            .field("LogFileReads", &self.LogFileReads)
            .field("LogFileReadBytes", &self.LogFileReadBytes)
            .field("LogFileWrites", &self.LogFileWrites)
            .field("LogFileWriteBytes", &self.LogFileWriteBytes)
            .field("Allocate", &self.Allocate)
            .field("DiskResourcesExhausted", &self.DiskResourcesExhausted)
            .finish()
    }
}
impl ::core::default::Default for NTFS_STATISTICS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Calls == other.Calls && self.Clusters == other.Clusters && self.Hints == other.Hints && self.RunsReturned == other.RunsReturned && self.HintsHonored == other.HintsHonored && self.HintsClusters == other.HintsClusters && self.Cache == other.Cache && self.CacheClusters == other.CacheClusters && self.CacheMiss == other.CacheMiss && self.CacheMissClusters == other.CacheMissClusters
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_0 {}
impl ::core::fmt::Debug for NTFS_STATISTICS_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_0").field("Calls", &self.Calls).field("Clusters", &self.Clusters).field("Hints", &self.Hints).field("RunsReturned", &self.RunsReturned).field("HintsHonored", &self.HintsHonored).field("HintsClusters", &self.HintsClusters).field("Cache", &self.Cache).field("CacheClusters", &self.CacheClusters).field("CacheMiss", &self.CacheMiss).field("CacheMissClusters", &self.CacheMissClusters).finish()
    }
}
impl ::core::default::Default for NTFS_STATISTICS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Write == other.Write && self.Create == other.Create && self.SetInfo == other.SetInfo
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_1 {}
impl ::core::fmt::Debug for NTFS_STATISTICS_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_1").field("Write", &self.Write).field("Create", &self.Create).field("SetInfo", &self.SetInfo).finish()
    }
}
impl ::core::default::Default for NTFS_STATISTICS_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Write == other.Write && self.Create == other.Create && self.SetInfo == other.SetInfo && self.Flush == other.Flush
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_2 {}
impl ::core::fmt::Debug for NTFS_STATISTICS_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_2").field("Write", &self.Write).field("Create", &self.Create).field("SetInfo", &self.SetInfo).field("Flush", &self.Flush).finish()
    }
}
impl ::core::default::Default for NTFS_STATISTICS_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Write == other.Write && self.Create == other.Create && self.SetInfo == other.SetInfo && self.Flush == other.Flush
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_3 {}
impl ::core::fmt::Debug for NTFS_STATISTICS_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_3").field("Write", &self.Write).field("Create", &self.Create).field("SetInfo", &self.SetInfo).field("Flush", &self.Flush).finish()
    }
}
impl ::core::default::Default for NTFS_STATISTICS_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_4 {
    fn eq(&self, other: &Self) -> bool {
        self.Write == other.Write && self.Create == other.Create && self.SetInfo == other.SetInfo && self.Flush == other.Flush
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_4 {}
impl ::core::fmt::Debug for NTFS_STATISTICS_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_4").field("Write", &self.Write).field("Create", &self.Create).field("SetInfo", &self.SetInfo).field("Flush", &self.Flush).finish()
    }
}
impl ::core::default::Default for NTFS_STATISTICS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_EX {
    fn eq(&self, other: &Self) -> bool {
        self.LogFileFullExceptions == other.LogFileFullExceptions
            && self.OtherExceptions == other.OtherExceptions
            && self.MftReads == other.MftReads
            && self.MftReadBytes == other.MftReadBytes
            && self.MftWrites == other.MftWrites
            && self.MftWriteBytes == other.MftWriteBytes
            && self.MftWritesUserLevel == other.MftWritesUserLevel
            && self.MftWritesFlushForLogFileFull == other.MftWritesFlushForLogFileFull
            && self.MftWritesLazyWriter == other.MftWritesLazyWriter
            && self.MftWritesUserRequest == other.MftWritesUserRequest
            && self.Mft2Writes == other.Mft2Writes
            && self.Mft2WriteBytes == other.Mft2WriteBytes
            && self.Mft2WritesUserLevel == other.Mft2WritesUserLevel
            && self.Mft2WritesFlushForLogFileFull == other.Mft2WritesFlushForLogFileFull
            && self.Mft2WritesLazyWriter == other.Mft2WritesLazyWriter
            && self.Mft2WritesUserRequest == other.Mft2WritesUserRequest
            && self.RootIndexReads == other.RootIndexReads
            && self.RootIndexReadBytes == other.RootIndexReadBytes
            && self.RootIndexWrites == other.RootIndexWrites
            && self.RootIndexWriteBytes == other.RootIndexWriteBytes
            && self.BitmapReads == other.BitmapReads
            && self.BitmapReadBytes == other.BitmapReadBytes
            && self.BitmapWrites == other.BitmapWrites
            && self.BitmapWriteBytes == other.BitmapWriteBytes
            && self.BitmapWritesFlushForLogFileFull == other.BitmapWritesFlushForLogFileFull
            && self.BitmapWritesLazyWriter == other.BitmapWritesLazyWriter
            && self.BitmapWritesUserRequest == other.BitmapWritesUserRequest
            && self.BitmapWritesUserLevel == other.BitmapWritesUserLevel
            && self.MftBitmapReads == other.MftBitmapReads
            && self.MftBitmapReadBytes == other.MftBitmapReadBytes
            && self.MftBitmapWrites == other.MftBitmapWrites
            && self.MftBitmapWriteBytes == other.MftBitmapWriteBytes
            && self.MftBitmapWritesFlushForLogFileFull == other.MftBitmapWritesFlushForLogFileFull
            && self.MftBitmapWritesLazyWriter == other.MftBitmapWritesLazyWriter
            && self.MftBitmapWritesUserRequest == other.MftBitmapWritesUserRequest
            && self.MftBitmapWritesUserLevel == other.MftBitmapWritesUserLevel
            && self.UserIndexReads == other.UserIndexReads
            && self.UserIndexReadBytes == other.UserIndexReadBytes
            && self.UserIndexWrites == other.UserIndexWrites
            && self.UserIndexWriteBytes == other.UserIndexWriteBytes
            && self.LogFileReads == other.LogFileReads
            && self.LogFileReadBytes == other.LogFileReadBytes
            && self.LogFileWrites == other.LogFileWrites
            && self.LogFileWriteBytes == other.LogFileWriteBytes
            && self.Allocate == other.Allocate
            && self.DiskResourcesExhausted == other.DiskResourcesExhausted
            && self.VolumeTrimCount == other.VolumeTrimCount
            && self.VolumeTrimTime == other.VolumeTrimTime
            && self.VolumeTrimByteCount == other.VolumeTrimByteCount
            && self.FileLevelTrimCount == other.FileLevelTrimCount
            && self.FileLevelTrimTime == other.FileLevelTrimTime
            && self.FileLevelTrimByteCount == other.FileLevelTrimByteCount
            && self.VolumeTrimSkippedCount == other.VolumeTrimSkippedCount
            && self.VolumeTrimSkippedByteCount == other.VolumeTrimSkippedByteCount
            && self.NtfsFillStatInfoFromMftRecordCalledCount == other.NtfsFillStatInfoFromMftRecordCalledCount
            && self.NtfsFillStatInfoFromMftRecordBailedBecauseOfAttributeListCount == other.NtfsFillStatInfoFromMftRecordBailedBecauseOfAttributeListCount
            && self.NtfsFillStatInfoFromMftRecordBailedBecauseOfNonResReparsePointCount == other.NtfsFillStatInfoFromMftRecordBailedBecauseOfNonResReparsePointCount
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_EX {}
impl ::core::fmt::Debug for NTFS_STATISTICS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_EX")
            .field("LogFileFullExceptions", &self.LogFileFullExceptions)
            .field("OtherExceptions", &self.OtherExceptions)
            .field("MftReads", &self.MftReads)
            .field("MftReadBytes", &self.MftReadBytes)
            .field("MftWrites", &self.MftWrites)
            .field("MftWriteBytes", &self.MftWriteBytes)
            .field("MftWritesUserLevel", &self.MftWritesUserLevel)
            .field("MftWritesFlushForLogFileFull", &self.MftWritesFlushForLogFileFull)
            .field("MftWritesLazyWriter", &self.MftWritesLazyWriter)
            .field("MftWritesUserRequest", &self.MftWritesUserRequest)
            .field("Mft2Writes", &self.Mft2Writes)
            .field("Mft2WriteBytes", &self.Mft2WriteBytes)
            .field("Mft2WritesUserLevel", &self.Mft2WritesUserLevel)
            .field("Mft2WritesFlushForLogFileFull", &self.Mft2WritesFlushForLogFileFull)
            .field("Mft2WritesLazyWriter", &self.Mft2WritesLazyWriter)
            .field("Mft2WritesUserRequest", &self.Mft2WritesUserRequest)
            .field("RootIndexReads", &self.RootIndexReads)
            .field("RootIndexReadBytes", &self.RootIndexReadBytes)
            .field("RootIndexWrites", &self.RootIndexWrites)
            .field("RootIndexWriteBytes", &self.RootIndexWriteBytes)
            .field("BitmapReads", &self.BitmapReads)
            .field("BitmapReadBytes", &self.BitmapReadBytes)
            .field("BitmapWrites", &self.BitmapWrites)
            .field("BitmapWriteBytes", &self.BitmapWriteBytes)
            .field("BitmapWritesFlushForLogFileFull", &self.BitmapWritesFlushForLogFileFull)
            .field("BitmapWritesLazyWriter", &self.BitmapWritesLazyWriter)
            .field("BitmapWritesUserRequest", &self.BitmapWritesUserRequest)
            .field("BitmapWritesUserLevel", &self.BitmapWritesUserLevel)
            .field("MftBitmapReads", &self.MftBitmapReads)
            .field("MftBitmapReadBytes", &self.MftBitmapReadBytes)
            .field("MftBitmapWrites", &self.MftBitmapWrites)
            .field("MftBitmapWriteBytes", &self.MftBitmapWriteBytes)
            .field("MftBitmapWritesFlushForLogFileFull", &self.MftBitmapWritesFlushForLogFileFull)
            .field("MftBitmapWritesLazyWriter", &self.MftBitmapWritesLazyWriter)
            .field("MftBitmapWritesUserRequest", &self.MftBitmapWritesUserRequest)
            .field("MftBitmapWritesUserLevel", &self.MftBitmapWritesUserLevel)
            .field("UserIndexReads", &self.UserIndexReads)
            .field("UserIndexReadBytes", &self.UserIndexReadBytes)
            .field("UserIndexWrites", &self.UserIndexWrites)
            .field("UserIndexWriteBytes", &self.UserIndexWriteBytes)
            .field("LogFileReads", &self.LogFileReads)
            .field("LogFileReadBytes", &self.LogFileReadBytes)
            .field("LogFileWrites", &self.LogFileWrites)
            .field("LogFileWriteBytes", &self.LogFileWriteBytes)
            .field("Allocate", &self.Allocate)
            .field("DiskResourcesExhausted", &self.DiskResourcesExhausted)
            .field("VolumeTrimCount", &self.VolumeTrimCount)
            .field("VolumeTrimTime", &self.VolumeTrimTime)
            .field("VolumeTrimByteCount", &self.VolumeTrimByteCount)
            .field("FileLevelTrimCount", &self.FileLevelTrimCount)
            .field("FileLevelTrimTime", &self.FileLevelTrimTime)
            .field("FileLevelTrimByteCount", &self.FileLevelTrimByteCount)
            .field("VolumeTrimSkippedCount", &self.VolumeTrimSkippedCount)
            .field("VolumeTrimSkippedByteCount", &self.VolumeTrimSkippedByteCount)
            .field("NtfsFillStatInfoFromMftRecordCalledCount", &self.NtfsFillStatInfoFromMftRecordCalledCount)
            .field("NtfsFillStatInfoFromMftRecordBailedBecauseOfAttributeListCount", &self.NtfsFillStatInfoFromMftRecordBailedBecauseOfAttributeListCount)
            .field("NtfsFillStatInfoFromMftRecordBailedBecauseOfNonResReparsePointCount", &self.NtfsFillStatInfoFromMftRecordBailedBecauseOfNonResReparsePointCount)
            .finish()
    }
}
impl ::core::default::Default for NTFS_STATISTICS_EX_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_EX_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Calls == other.Calls && self.RunsReturned == other.RunsReturned && self.Hints == other.Hints && self.HintsHonored == other.HintsHonored && self.Cache == other.Cache && self.CacheMiss == other.CacheMiss && self.Clusters == other.Clusters && self.HintsClusters == other.HintsClusters && self.CacheClusters == other.CacheClusters && self.CacheMissClusters == other.CacheMissClusters
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_EX_0 {}
impl ::core::fmt::Debug for NTFS_STATISTICS_EX_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_EX_0").field("Calls", &self.Calls).field("RunsReturned", &self.RunsReturned).field("Hints", &self.Hints).field("HintsHonored", &self.HintsHonored).field("Cache", &self.Cache).field("CacheMiss", &self.CacheMiss).field("Clusters", &self.Clusters).field("HintsClusters", &self.HintsClusters).field("CacheClusters", &self.CacheClusters).field("CacheMissClusters", &self.CacheMissClusters).finish()
    }
}
impl ::core::default::Default for NTFS_STATISTICS_EX_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_EX_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Write == other.Write && self.Create == other.Create && self.SetInfo == other.SetInfo && self.Flush == other.Flush
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_EX_1 {}
impl ::core::fmt::Debug for NTFS_STATISTICS_EX_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_EX_1").field("Write", &self.Write).field("Create", &self.Create).field("SetInfo", &self.SetInfo).field("Flush", &self.Flush).finish()
    }
}
impl ::core::default::Default for NTFS_STATISTICS_EX_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_EX_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Write == other.Write && self.Create == other.Create && self.SetInfo == other.SetInfo && self.Flush == other.Flush
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_EX_2 {}
impl ::core::fmt::Debug for NTFS_STATISTICS_EX_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_EX_2").field("Write", &self.Write).field("Create", &self.Create).field("SetInfo", &self.SetInfo).field("Flush", &self.Flush).finish()
    }
}
impl ::core::default::Default for NTFS_STATISTICS_EX_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_EX_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Write == other.Write && self.Create == other.Create && self.SetInfo == other.SetInfo && self.Flush == other.Flush
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_EX_3 {}
impl ::core::fmt::Debug for NTFS_STATISTICS_EX_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_EX_3").field("Write", &self.Write).field("Create", &self.Create).field("SetInfo", &self.SetInfo).field("Flush", &self.Flush).finish()
    }
}
impl ::core::default::Default for NTFS_STATISTICS_EX_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTFS_STATISTICS_EX_4 {
    fn eq(&self, other: &Self) -> bool {
        self.Write == other.Write && self.Create == other.Create && self.SetInfo == other.SetInfo && self.Flush == other.Flush
    }
}
impl ::core::cmp::Eq for NTFS_STATISTICS_EX_4 {}
impl ::core::fmt::Debug for NTFS_STATISTICS_EX_4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_STATISTICS_EX_4").field("Write", &self.Write).field("Create", &self.Create).field("SetInfo", &self.SetInfo).field("Flush", &self.Flush).finish()
    }
}
impl ::core::default::Default for NTFS_VOLUME_DATA_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NTFS_VOLUME_DATA_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.VolumeSerialNumber == other.VolumeSerialNumber && self.NumberSectors == other.NumberSectors && self.TotalClusters == other.TotalClusters && self.FreeClusters == other.FreeClusters && self.TotalReserved == other.TotalReserved && self.BytesPerSector == other.BytesPerSector && self.BytesPerCluster == other.BytesPerCluster && self.BytesPerFileRecordSegment == other.BytesPerFileRecordSegment && self.ClustersPerFileRecordSegment == other.ClustersPerFileRecordSegment && self.MftValidDataLength == other.MftValidDataLength && self.MftStartLcn == other.MftStartLcn && self.Mft2StartLcn == other.Mft2StartLcn && self.MftZoneStart == other.MftZoneStart && self.MftZoneEnd == other.MftZoneEnd
    }
}
impl ::core::cmp::Eq for NTFS_VOLUME_DATA_BUFFER {}
impl ::core::fmt::Debug for NTFS_VOLUME_DATA_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NTFS_VOLUME_DATA_BUFFER")
            .field("VolumeSerialNumber", &self.VolumeSerialNumber)
            .field("NumberSectors", &self.NumberSectors)
            .field("TotalClusters", &self.TotalClusters)
            .field("FreeClusters", &self.FreeClusters)
            .field("TotalReserved", &self.TotalReserved)
            .field("BytesPerSector", &self.BytesPerSector)
            .field("BytesPerCluster", &self.BytesPerCluster)
            .field("BytesPerFileRecordSegment", &self.BytesPerFileRecordSegment)
            .field("ClustersPerFileRecordSegment", &self.ClustersPerFileRecordSegment)
            .field("MftValidDataLength", &self.MftValidDataLength)
            .field("MftStartLcn", &self.MftStartLcn)
            .field("Mft2StartLcn", &self.Mft2StartLcn)
            .field("MftZoneStart", &self.MftZoneStart)
            .field("MftZoneEnd", &self.MftZoneEnd)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PARTITION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.StartingOffset == other.StartingOffset && self.PartitionLength == other.PartitionLength && self.HiddenSectors == other.HiddenSectors && self.PartitionNumber == other.PartitionNumber && self.PartitionType == other.PartitionType && self.BootIndicator == other.BootIndicator && self.RecognizedPartition == other.RecognizedPartition && self.RewritePartition == other.RewritePartition
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PARTITION_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PARTITION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PARTITION_INFORMATION").field("StartingOffset", &self.StartingOffset).field("PartitionLength", &self.PartitionLength).field("HiddenSectors", &self.HiddenSectors).field("PartitionNumber", &self.PartitionNumber).field("PartitionType", &self.PartitionType).field("BootIndicator", &self.BootIndicator).field("RecognizedPartition", &self.RecognizedPartition).field("RewritePartition", &self.RewritePartition).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PARTITION_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PARTITION_INFORMATION_GPT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PARTITION_INFORMATION_GPT {
    fn eq(&self, other: &Self) -> bool {
        self.PartitionType == other.PartitionType && self.PartitionId == other.PartitionId && self.Attributes == other.Attributes && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for PARTITION_INFORMATION_GPT {}
impl ::core::fmt::Debug for PARTITION_INFORMATION_GPT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PARTITION_INFORMATION_GPT").field("PartitionType", &self.PartitionType).field("PartitionId", &self.PartitionId).field("Attributes", &self.Attributes).field("Name", &self.Name).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PARTITION_INFORMATION_MBR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PARTITION_INFORMATION_MBR {
    fn eq(&self, other: &Self) -> bool {
        self.PartitionType == other.PartitionType && self.BootIndicator == other.BootIndicator && self.RecognizedPartition == other.RecognizedPartition && self.HiddenSectors == other.HiddenSectors && self.PartitionId == other.PartitionId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PARTITION_INFORMATION_MBR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PARTITION_INFORMATION_MBR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PARTITION_INFORMATION_MBR").field("PartitionType", &self.PartitionType).field("BootIndicator", &self.BootIndicator).field("RecognizedPartition", &self.RecognizedPartition).field("HiddenSectors", &self.HiddenSectors).field("PartitionId", &self.PartitionId).finish()
    }
}
impl ::core::default::Default for PARTITION_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PARTITION_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PARTITION_STYLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for PATHNAME_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PATHNAME_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.PathNameLength == other.PathNameLength && self.Name == other.Name
    }
}
impl ::core::cmp::Eq for PATHNAME_BUFFER {}
impl ::core::fmt::Debug for PATHNAME_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PATHNAME_BUFFER").field("PathNameLength", &self.PathNameLength).field("Name", &self.Name).finish()
    }
}
impl ::core::default::Default for PERF_BIN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PERF_BIN {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfBins == other.NumberOfBins && self.TypeOfBin == other.TypeOfBin && self.BinsRanges == other.BinsRanges
    }
}
impl ::core::cmp::Eq for PERF_BIN {}
impl ::core::fmt::Debug for PERF_BIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PERF_BIN").field("NumberOfBins", &self.NumberOfBins).field("TypeOfBin", &self.TypeOfBin).field("BinsRanges", &self.BinsRanges).finish()
    }
}
impl ::core::default::Default for PERSISTENT_RESERVE_COMMAND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for PHYSICAL_ELEMENT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PHYSICAL_ELEMENT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.DescriptorCount == other.DescriptorCount && self.ReturnedDescriptorCount == other.ReturnedDescriptorCount && self.ElementIdentifierBeingDepoped == other.ElementIdentifierBeingDepoped && self.Reserved == other.Reserved && self.Descriptors == other.Descriptors
    }
}
impl ::core::cmp::Eq for PHYSICAL_ELEMENT_STATUS {}
impl ::core::fmt::Debug for PHYSICAL_ELEMENT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PHYSICAL_ELEMENT_STATUS").field("Version", &self.Version).field("Size", &self.Size).field("DescriptorCount", &self.DescriptorCount).field("ReturnedDescriptorCount", &self.ReturnedDescriptorCount).field("ElementIdentifierBeingDepoped", &self.ElementIdentifierBeingDepoped).field("Reserved", &self.Reserved).field("Descriptors", &self.Descriptors).finish()
    }
}
impl ::core::default::Default for PHYSICAL_ELEMENT_STATUS_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PHYSICAL_ELEMENT_STATUS_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.ElementIdentifier == other.ElementIdentifier && self.PhysicalElementType == other.PhysicalElementType && self.PhysicalElementHealth == other.PhysicalElementHealth && self.Reserved1 == other.Reserved1 && self.AssociatedCapacity == other.AssociatedCapacity && self.Reserved2 == other.Reserved2
    }
}
impl ::core::cmp::Eq for PHYSICAL_ELEMENT_STATUS_DESCRIPTOR {}
impl ::core::fmt::Debug for PHYSICAL_ELEMENT_STATUS_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PHYSICAL_ELEMENT_STATUS_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("ElementIdentifier", &self.ElementIdentifier).field("PhysicalElementType", &self.PhysicalElementType).field("PhysicalElementHealth", &self.PhysicalElementHealth).field("Reserved1", &self.Reserved1).field("AssociatedCapacity", &self.AssociatedCapacity).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::default::Default for PHYSICAL_ELEMENT_STATUS_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PHYSICAL_ELEMENT_STATUS_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.StartingElement == other.StartingElement && self.Filter == other.Filter && self.ReportType == other.ReportType && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for PHYSICAL_ELEMENT_STATUS_REQUEST {}
impl ::core::fmt::Debug for PHYSICAL_ELEMENT_STATUS_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PHYSICAL_ELEMENT_STATUS_REQUEST").field("Version", &self.Version).field("Size", &self.Size).field("StartingElement", &self.StartingElement).field("Filter", &self.Filter).field("ReportType", &self.ReportType).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for PLEX_READ_DATA_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PLEX_READ_DATA_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.ByteOffset == other.ByteOffset && self.ByteLength == other.ByteLength && self.PlexNumber == other.PlexNumber
    }
}
impl ::core::cmp::Eq for PLEX_READ_DATA_REQUEST {}
impl ::core::fmt::Debug for PLEX_READ_DATA_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PLEX_READ_DATA_REQUEST").field("ByteOffset", &self.ByteOffset).field("ByteLength", &self.ByteLength).field("PlexNumber", &self.PlexNumber).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PREVENT_MEDIA_REMOVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PREVENT_MEDIA_REMOVAL {
    fn eq(&self, other: &Self) -> bool {
        self.PreventMediaRemoval == other.PreventMediaRemoval
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PREVENT_MEDIA_REMOVAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PREVENT_MEDIA_REMOVAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PREVENT_MEDIA_REMOVAL").field("PreventMediaRemoval", &self.PreventMediaRemoval).finish()
    }
}
impl ::core::default::Default for QUERY_BAD_RANGES_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QUERY_BAD_RANGES_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.NumRanges == other.NumRanges && self.Ranges == other.Ranges
    }
}
impl ::core::cmp::Eq for QUERY_BAD_RANGES_INPUT {}
impl ::core::fmt::Debug for QUERY_BAD_RANGES_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERY_BAD_RANGES_INPUT").field("Flags", &self.Flags).field("NumRanges", &self.NumRanges).field("Ranges", &self.Ranges).finish()
    }
}
impl ::core::default::Default for QUERY_BAD_RANGES_INPUT_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QUERY_BAD_RANGES_INPUT_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartOffset == other.StartOffset && self.LengthInBytes == other.LengthInBytes
    }
}
impl ::core::cmp::Eq for QUERY_BAD_RANGES_INPUT_RANGE {}
impl ::core::fmt::Debug for QUERY_BAD_RANGES_INPUT_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERY_BAD_RANGES_INPUT_RANGE").field("StartOffset", &self.StartOffset).field("LengthInBytes", &self.LengthInBytes).finish()
    }
}
impl ::core::default::Default for QUERY_BAD_RANGES_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QUERY_BAD_RANGES_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.NumBadRanges == other.NumBadRanges && self.NextOffsetToLookUp == other.NextOffsetToLookUp && self.BadRanges == other.BadRanges
    }
}
impl ::core::cmp::Eq for QUERY_BAD_RANGES_OUTPUT {}
impl ::core::fmt::Debug for QUERY_BAD_RANGES_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERY_BAD_RANGES_OUTPUT").field("Flags", &self.Flags).field("NumBadRanges", &self.NumBadRanges).field("NextOffsetToLookUp", &self.NextOffsetToLookUp).field("BadRanges", &self.BadRanges).finish()
    }
}
impl ::core::default::Default for QUERY_BAD_RANGES_OUTPUT_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QUERY_BAD_RANGES_OUTPUT_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Reserved == other.Reserved && self.StartOffset == other.StartOffset && self.LengthInBytes == other.LengthInBytes
    }
}
impl ::core::cmp::Eq for QUERY_BAD_RANGES_OUTPUT_RANGE {}
impl ::core::fmt::Debug for QUERY_BAD_RANGES_OUTPUT_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERY_BAD_RANGES_OUTPUT_RANGE").field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("StartOffset", &self.StartOffset).field("LengthInBytes", &self.LengthInBytes).finish()
    }
}
impl ::core::default::Default for QUERY_FILE_LAYOUT_FILTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for QUERY_FILE_LAYOUT_FILTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUERY_FILE_LAYOUT_FILTER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for QUERY_FILE_LAYOUT_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for QUERY_FILE_LAYOUT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for QUERY_FILE_LAYOUT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.FileEntryCount == other.FileEntryCount && self.FirstFileOffset == other.FirstFileOffset && self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for QUERY_FILE_LAYOUT_OUTPUT {}
impl ::core::fmt::Debug for QUERY_FILE_LAYOUT_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERY_FILE_LAYOUT_OUTPUT").field("FileEntryCount", &self.FileEntryCount).field("FirstFileOffset", &self.FirstFileOffset).field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for READ_ELEMENT_ADDRESS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for READ_ELEMENT_ADDRESS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfElements == other.NumberOfElements && self.ElementStatus == other.ElementStatus
    }
}
impl ::core::cmp::Eq for READ_ELEMENT_ADDRESS_INFO {}
impl ::core::fmt::Debug for READ_ELEMENT_ADDRESS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READ_ELEMENT_ADDRESS_INFO").field("NumberOfElements", &self.NumberOfElements).field("ElementStatus", &self.ElementStatus).finish()
    }
}
impl ::core::default::Default for READ_FILE_USN_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for READ_FILE_USN_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.MinMajorVersion == other.MinMajorVersion && self.MaxMajorVersion == other.MaxMajorVersion
    }
}
impl ::core::cmp::Eq for READ_FILE_USN_DATA {}
impl ::core::fmt::Debug for READ_FILE_USN_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READ_FILE_USN_DATA").field("MinMajorVersion", &self.MinMajorVersion).field("MaxMajorVersion", &self.MaxMajorVersion).finish()
    }
}
impl ::core::default::Default for READ_USN_JOURNAL_DATA_V0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for READ_USN_JOURNAL_DATA_V0 {
    fn eq(&self, other: &Self) -> bool {
        self.StartUsn == other.StartUsn && self.ReasonMask == other.ReasonMask && self.ReturnOnlyOnClose == other.ReturnOnlyOnClose && self.Timeout == other.Timeout && self.BytesToWaitFor == other.BytesToWaitFor && self.UsnJournalID == other.UsnJournalID
    }
}
impl ::core::cmp::Eq for READ_USN_JOURNAL_DATA_V0 {}
impl ::core::fmt::Debug for READ_USN_JOURNAL_DATA_V0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READ_USN_JOURNAL_DATA_V0").field("StartUsn", &self.StartUsn).field("ReasonMask", &self.ReasonMask).field("ReturnOnlyOnClose", &self.ReturnOnlyOnClose).field("Timeout", &self.Timeout).field("BytesToWaitFor", &self.BytesToWaitFor).field("UsnJournalID", &self.UsnJournalID).finish()
    }
}
impl ::core::default::Default for READ_USN_JOURNAL_DATA_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for READ_USN_JOURNAL_DATA_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.StartUsn == other.StartUsn && self.ReasonMask == other.ReasonMask && self.ReturnOnlyOnClose == other.ReturnOnlyOnClose && self.Timeout == other.Timeout && self.BytesToWaitFor == other.BytesToWaitFor && self.UsnJournalID == other.UsnJournalID && self.MinMajorVersion == other.MinMajorVersion && self.MaxMajorVersion == other.MaxMajorVersion
    }
}
impl ::core::cmp::Eq for READ_USN_JOURNAL_DATA_V1 {}
impl ::core::fmt::Debug for READ_USN_JOURNAL_DATA_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("READ_USN_JOURNAL_DATA_V1").field("StartUsn", &self.StartUsn).field("ReasonMask", &self.ReasonMask).field("ReturnOnlyOnClose", &self.ReturnOnlyOnClose).field("Timeout", &self.Timeout).field("BytesToWaitFor", &self.BytesToWaitFor).field("UsnJournalID", &self.UsnJournalID).field("MinMajorVersion", &self.MinMajorVersion).field("MaxMajorVersion", &self.MaxMajorVersion).finish()
    }
}
impl ::core::default::Default for REASSIGN_BLOCKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REASSIGN_BLOCKS {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved && self.Count == other.Count && self.BlockNumber == other.BlockNumber
    }
}
impl ::core::cmp::Eq for REASSIGN_BLOCKS {}
impl ::core::fmt::Debug for REASSIGN_BLOCKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REASSIGN_BLOCKS").field("Reserved", &self.Reserved).field("Count", &self.Count).field("BlockNumber", &self.BlockNumber).finish()
    }
}
impl ::core::default::Default for REASSIGN_BLOCKS_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for REFS_SMR_VOLUME_GC_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REFS_SMR_VOLUME_GC_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REFS_SMR_VOLUME_GC_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for REFS_SMR_VOLUME_GC_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REFS_SMR_VOLUME_GC_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REFS_SMR_VOLUME_GC_METHOD").field(&self.0).finish()
    }
}
impl ::core::default::Default for REFS_SMR_VOLUME_GC_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REFS_SMR_VOLUME_GC_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.Action == other.Action && self.Method == other.Method && self.IoGranularity == other.IoGranularity && self.CompressionFormat == other.CompressionFormat && self.Unused == other.Unused
    }
}
impl ::core::cmp::Eq for REFS_SMR_VOLUME_GC_PARAMETERS {}
impl ::core::fmt::Debug for REFS_SMR_VOLUME_GC_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REFS_SMR_VOLUME_GC_PARAMETERS").field("Version", &self.Version).field("Flags", &self.Flags).field("Action", &self.Action).field("Method", &self.Method).field("IoGranularity", &self.IoGranularity).field("CompressionFormat", &self.CompressionFormat).field("Unused", &self.Unused).finish()
    }
}
impl ::core::default::Default for REFS_SMR_VOLUME_GC_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for REFS_SMR_VOLUME_GC_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REFS_SMR_VOLUME_GC_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for REFS_SMR_VOLUME_INFO_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REFS_SMR_VOLUME_INFO_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.SizeOfRandomlyWritableTier == other.SizeOfRandomlyWritableTier && self.FreeSpaceInRandomlyWritableTier == other.FreeSpaceInRandomlyWritableTier && self.SizeofSMRTier == other.SizeofSMRTier && self.FreeSpaceInSMRTier == other.FreeSpaceInSMRTier && self.UsableFreeSpaceInSMRTier == other.UsableFreeSpaceInSMRTier && self.VolumeGcState == other.VolumeGcState && self.VolumeGcLastStatus == other.VolumeGcLastStatus && self.CurrentGcBandFillPercentage == other.CurrentGcBandFillPercentage && self.Unused == other.Unused
    }
}
impl ::core::cmp::Eq for REFS_SMR_VOLUME_INFO_OUTPUT {}
impl ::core::fmt::Debug for REFS_SMR_VOLUME_INFO_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REFS_SMR_VOLUME_INFO_OUTPUT")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("SizeOfRandomlyWritableTier", &self.SizeOfRandomlyWritableTier)
            .field("FreeSpaceInRandomlyWritableTier", &self.FreeSpaceInRandomlyWritableTier)
            .field("SizeofSMRTier", &self.SizeofSMRTier)
            .field("FreeSpaceInSMRTier", &self.FreeSpaceInSMRTier)
            .field("UsableFreeSpaceInSMRTier", &self.UsableFreeSpaceInSMRTier)
            .field("VolumeGcState", &self.VolumeGcState)
            .field("VolumeGcLastStatus", &self.VolumeGcLastStatus)
            .field("CurrentGcBandFillPercentage", &self.CurrentGcBandFillPercentage)
            .field("Unused", &self.Unused)
            .finish()
    }
}
impl ::core::default::Default for REFS_VOLUME_DATA_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REFS_VOLUME_DATA_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.ByteCount == other.ByteCount
            && self.MajorVersion == other.MajorVersion
            && self.MinorVersion == other.MinorVersion
            && self.BytesPerPhysicalSector == other.BytesPerPhysicalSector
            && self.VolumeSerialNumber == other.VolumeSerialNumber
            && self.NumberSectors == other.NumberSectors
            && self.TotalClusters == other.TotalClusters
            && self.FreeClusters == other.FreeClusters
            && self.TotalReserved == other.TotalReserved
            && self.BytesPerSector == other.BytesPerSector
            && self.BytesPerCluster == other.BytesPerCluster
            && self.MaximumSizeOfResidentFile == other.MaximumSizeOfResidentFile
            && self.FastTierDataFillRatio == other.FastTierDataFillRatio
            && self.SlowTierDataFillRatio == other.SlowTierDataFillRatio
            && self.DestagesFastTierToSlowTierRate == other.DestagesFastTierToSlowTierRate
            && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for REFS_VOLUME_DATA_BUFFER {}
impl ::core::fmt::Debug for REFS_VOLUME_DATA_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REFS_VOLUME_DATA_BUFFER")
            .field("ByteCount", &self.ByteCount)
            .field("MajorVersion", &self.MajorVersion)
            .field("MinorVersion", &self.MinorVersion)
            .field("BytesPerPhysicalSector", &self.BytesPerPhysicalSector)
            .field("VolumeSerialNumber", &self.VolumeSerialNumber)
            .field("NumberSectors", &self.NumberSectors)
            .field("TotalClusters", &self.TotalClusters)
            .field("FreeClusters", &self.FreeClusters)
            .field("TotalReserved", &self.TotalReserved)
            .field("BytesPerSector", &self.BytesPerSector)
            .field("BytesPerCluster", &self.BytesPerCluster)
            .field("MaximumSizeOfResidentFile", &self.MaximumSizeOfResidentFile)
            .field("FastTierDataFillRatio", &self.FastTierDataFillRatio)
            .field("SlowTierDataFillRatio", &self.SlowTierDataFillRatio)
            .field("DestagesFastTierToSlowTierRate", &self.DestagesFastTierToSlowTierRate)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::default::Default for REMOVE_ELEMENT_AND_TRUNCATE_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REMOVE_ELEMENT_AND_TRUNCATE_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.RequestCapacity == other.RequestCapacity && self.ElementIdentifier == other.ElementIdentifier && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for REMOVE_ELEMENT_AND_TRUNCATE_REQUEST {}
impl ::core::fmt::Debug for REMOVE_ELEMENT_AND_TRUNCATE_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REMOVE_ELEMENT_AND_TRUNCATE_REQUEST").field("Version", &self.Version).field("Size", &self.Size).field("RequestCapacity", &self.RequestCapacity).field("ElementIdentifier", &self.ElementIdentifier).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for REPAIR_COPIES_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REPAIR_COPIES_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Flags == other.Flags && self.FileOffset == other.FileOffset && self.Length == other.Length && self.SourceCopy == other.SourceCopy && self.NumberOfRepairCopies == other.NumberOfRepairCopies && self.RepairCopies == other.RepairCopies
    }
}
impl ::core::cmp::Eq for REPAIR_COPIES_INPUT {}
impl ::core::fmt::Debug for REPAIR_COPIES_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPAIR_COPIES_INPUT").field("Size", &self.Size).field("Flags", &self.Flags).field("FileOffset", &self.FileOffset).field("Length", &self.Length).field("SourceCopy", &self.SourceCopy).field("NumberOfRepairCopies", &self.NumberOfRepairCopies).field("RepairCopies", &self.RepairCopies).finish()
    }
}
impl ::core::default::Default for REPAIR_COPIES_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REPAIR_COPIES_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Status == other.Status && self.ResumeFileOffset == other.ResumeFileOffset
    }
}
impl ::core::cmp::Eq for REPAIR_COPIES_OUTPUT {}
impl ::core::fmt::Debug for REPAIR_COPIES_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REPAIR_COPIES_OUTPUT").field("Size", &self.Size).field("Status", &self.Status).field("ResumeFileOffset", &self.ResumeFileOffset).finish()
    }
}
impl ::core::default::Default for REQUEST_OPLOCK_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REQUEST_OPLOCK_INPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.StructureVersion == other.StructureVersion && self.StructureLength == other.StructureLength && self.RequestedOplockLevel == other.RequestedOplockLevel && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for REQUEST_OPLOCK_INPUT_BUFFER {}
impl ::core::fmt::Debug for REQUEST_OPLOCK_INPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUEST_OPLOCK_INPUT_BUFFER").field("StructureVersion", &self.StructureVersion).field("StructureLength", &self.StructureLength).field("RequestedOplockLevel", &self.RequestedOplockLevel).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for REQUEST_OPLOCK_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REQUEST_OPLOCK_OUTPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.StructureVersion == other.StructureVersion && self.StructureLength == other.StructureLength && self.OriginalOplockLevel == other.OriginalOplockLevel && self.NewOplockLevel == other.NewOplockLevel && self.Flags == other.Flags && self.AccessMode == other.AccessMode && self.ShareMode == other.ShareMode
    }
}
impl ::core::cmp::Eq for REQUEST_OPLOCK_OUTPUT_BUFFER {}
impl ::core::fmt::Debug for REQUEST_OPLOCK_OUTPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUEST_OPLOCK_OUTPUT_BUFFER").field("StructureVersion", &self.StructureVersion).field("StructureLength", &self.StructureLength).field("OriginalOplockLevel", &self.OriginalOplockLevel).field("NewOplockLevel", &self.NewOplockLevel).field("Flags", &self.Flags).field("AccessMode", &self.AccessMode).field("ShareMode", &self.ShareMode).finish()
    }
}
impl ::core::default::Default for REQUEST_RAW_ENCRYPTED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for REQUEST_RAW_ENCRYPTED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.FileOffset == other.FileOffset && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for REQUEST_RAW_ENCRYPTED_DATA {}
impl ::core::fmt::Debug for REQUEST_RAW_ENCRYPTED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REQUEST_RAW_ENCRYPTED_DATA").field("FileOffset", &self.FileOffset).field("Length", &self.Length).finish()
    }
}
impl ::core::default::Default for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.ExtentCount == other.ExtentCount && self.StartingVcn == other.StartingVcn && self.Extents == other.Extents
    }
}
impl ::core::cmp::Eq for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {}
impl ::core::fmt::Debug for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER").field("ExtentCount", &self.ExtentCount).field("StartingVcn", &self.StartingVcn).field("Extents", &self.Extents).finish()
    }
}
impl ::core::default::Default for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.NextVcn == other.NextVcn && self.Lcn == other.Lcn && self.ReferenceCount == other.ReferenceCount
    }
}
impl ::core::cmp::Eq for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {}
impl ::core::fmt::Debug for RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RETRIEVAL_POINTERS_AND_REFCOUNT_BUFFER_0").field("NextVcn", &self.NextVcn).field("Lcn", &self.Lcn).field("ReferenceCount", &self.ReferenceCount).finish()
    }
}
impl ::core::default::Default for RETRIEVAL_POINTERS_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RETRIEVAL_POINTERS_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.ExtentCount == other.ExtentCount && self.StartingVcn == other.StartingVcn && self.Extents == other.Extents
    }
}
impl ::core::cmp::Eq for RETRIEVAL_POINTERS_BUFFER {}
impl ::core::fmt::Debug for RETRIEVAL_POINTERS_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RETRIEVAL_POINTERS_BUFFER").field("ExtentCount", &self.ExtentCount).field("StartingVcn", &self.StartingVcn).field("Extents", &self.Extents).finish()
    }
}
impl ::core::default::Default for RETRIEVAL_POINTERS_BUFFER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RETRIEVAL_POINTERS_BUFFER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.NextVcn == other.NextVcn && self.Lcn == other.Lcn
    }
}
impl ::core::cmp::Eq for RETRIEVAL_POINTERS_BUFFER_0 {}
impl ::core::fmt::Debug for RETRIEVAL_POINTERS_BUFFER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RETRIEVAL_POINTERS_BUFFER_0").field("NextVcn", &self.NextVcn).field("Lcn", &self.Lcn).finish()
    }
}
impl ::core::default::Default for RETRIEVAL_POINTER_BASE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RETRIEVAL_POINTER_BASE {
    fn eq(&self, other: &Self) -> bool {
        self.FileAreaOffset == other.FileAreaOffset
    }
}
impl ::core::cmp::Eq for RETRIEVAL_POINTER_BASE {}
impl ::core::fmt::Debug for RETRIEVAL_POINTER_BASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RETRIEVAL_POINTER_BASE").field("FileAreaOffset", &self.FileAreaOffset).finish()
    }
}
impl ::core::default::Default for RETRIEVAL_POINTER_COUNT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for RETRIEVAL_POINTER_COUNT {
    fn eq(&self, other: &Self) -> bool {
        self.ExtentCount == other.ExtentCount
    }
}
impl ::core::cmp::Eq for RETRIEVAL_POINTER_COUNT {}
impl ::core::fmt::Debug for RETRIEVAL_POINTER_COUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RETRIEVAL_POINTER_COUNT").field("ExtentCount", &self.ExtentCount).finish()
    }
}
impl ::core::default::Default for SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.DeviceCount == other.DeviceCount && self.Devices == other.Devices
    }
}
impl ::core::cmp::Eq for SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO {}
impl ::core::fmt::Debug for SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_BUS_DEDICATED_MEMORY_DEVICES_INFO").field("Version", &self.Version).field("Size", &self.Size).field("DeviceCount", &self.DeviceCount).field("Devices", &self.Devices).finish()
    }
}
impl ::core::default::Default for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceGuid == other.DeviceGuid && self.DeviceNumber == other.DeviceNumber && self.Flags == other.Flags && self.DeviceSize == other.DeviceSize
    }
}
impl ::core::cmp::Eq for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO {}
impl ::core::fmt::Debug for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO").field("DeviceGuid", &self.DeviceGuid).field("DeviceNumber", &self.DeviceNumber).field("Flags", &self.Flags).field("DeviceSize", &self.DeviceSize).finish()
    }
}
impl ::core::default::Default for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0 {}
impl ::core::fmt::Debug for SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_BUS_DEDICATED_MEMORY_DEVICE_INFO_0").field("_bitfield", &self._bitfield).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCM_BUS_DEDICATED_MEMORY_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCM_BUS_DEDICATED_MEMORY_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.ActivateState == other.ActivateState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCM_BUS_DEDICATED_MEMORY_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCM_BUS_DEDICATED_MEMORY_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_BUS_DEDICATED_MEMORY_STATE").field("ActivateState", &self.ActivateState).finish()
    }
}
impl ::core::default::Default for SCM_BUS_FIRMWARE_ACTIVATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCM_BUS_FIRMWARE_ACTIVATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_BUS_FIRMWARE_ACTIVATION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCM_BUS_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCM_BUS_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_BUS_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCM_BUS_PROPERTY_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_BUS_PROPERTY_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.PropertyId == other.PropertyId && self.QueryType == other.QueryType && self.AdditionalParameters == other.AdditionalParameters
    }
}
impl ::core::cmp::Eq for SCM_BUS_PROPERTY_QUERY {}
impl ::core::fmt::Debug for SCM_BUS_PROPERTY_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_BUS_PROPERTY_QUERY").field("Version", &self.Version).field("Size", &self.Size).field("PropertyId", &self.PropertyId).field("QueryType", &self.QueryType).field("AdditionalParameters", &self.AdditionalParameters).finish()
    }
}
impl ::core::default::Default for SCM_BUS_PROPERTY_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_BUS_PROPERTY_SET {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.PropertyId == other.PropertyId && self.SetType == other.SetType && self.AdditionalParameters == other.AdditionalParameters
    }
}
impl ::core::cmp::Eq for SCM_BUS_PROPERTY_SET {}
impl ::core::fmt::Debug for SCM_BUS_PROPERTY_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_BUS_PROPERTY_SET").field("Version", &self.Version).field("Size", &self.Size).field("PropertyId", &self.PropertyId).field("SetType", &self.SetType).field("AdditionalParameters", &self.AdditionalParameters).finish()
    }
}
impl ::core::default::Default for SCM_BUS_QUERY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCM_BUS_QUERY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_BUS_QUERY_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.RuntimeFwActivationSupported == other.RuntimeFwActivationSupported && self.FirmwareActivationState == other.FirmwareActivationState && self.FirmwareActivationCapability == other.FirmwareActivationCapability && self.EstimatedFirmwareActivationTimeInUSecs == other.EstimatedFirmwareActivationTimeInUSecs && self.EstimatedProcessorAccessQuiesceTimeInUSecs == other.EstimatedProcessorAccessQuiesceTimeInUSecs && self.EstimatedIOAccessQuiesceTimeInUSecs == other.EstimatedIOAccessQuiesceTimeInUSecs && self.PlatformSupportedMaxIOAccessQuiesceTimeInUSecs == other.PlatformSupportedMaxIOAccessQuiesceTimeInUSecs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_BUS_RUNTIME_FW_ACTIVATION_INFO")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("RuntimeFwActivationSupported", &self.RuntimeFwActivationSupported)
            .field("FirmwareActivationState", &self.FirmwareActivationState)
            .field("FirmwareActivationCapability", &self.FirmwareActivationCapability)
            .field("EstimatedFirmwareActivationTimeInUSecs", &self.EstimatedFirmwareActivationTimeInUSecs)
            .field("EstimatedProcessorAccessQuiesceTimeInUSecs", &self.EstimatedProcessorAccessQuiesceTimeInUSecs)
            .field("EstimatedIOAccessQuiesceTimeInUSecs", &self.EstimatedIOAccessQuiesceTimeInUSecs)
            .field("PlatformSupportedMaxIOAccessQuiesceTimeInUSecs", &self.PlatformSupportedMaxIOAccessQuiesceTimeInUSecs)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_BUS_RUNTIME_FW_ACTIVATION_INFO_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for SCM_BUS_SET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCM_BUS_SET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_BUS_SET_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCM_INTERLEAVED_PD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_INTERLEAVED_PD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceHandle == other.DeviceHandle && self.DeviceGuid == other.DeviceGuid
    }
}
impl ::core::cmp::Eq for SCM_INTERLEAVED_PD_INFO {}
impl ::core::fmt::Debug for SCM_INTERLEAVED_PD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_INTERLEAVED_PD_INFO").field("DeviceHandle", &self.DeviceHandle).field("DeviceGuid", &self.DeviceGuid).finish()
    }
}
impl ::core::default::Default for SCM_LD_INTERLEAVE_SET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_LD_INTERLEAVE_SET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.InterleaveSetSize == other.InterleaveSetSize && self.InterleaveSet == other.InterleaveSet
    }
}
impl ::core::cmp::Eq for SCM_LD_INTERLEAVE_SET_INFO {}
impl ::core::fmt::Debug for SCM_LD_INTERLEAVE_SET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_LD_INTERLEAVE_SET_INFO").field("Version", &self.Version).field("Size", &self.Size).field("InterleaveSetSize", &self.InterleaveSetSize).field("InterleaveSet", &self.InterleaveSet).finish()
    }
}
impl ::core::default::Default for SCM_LOGICAL_DEVICES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_LOGICAL_DEVICES {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.DeviceCount == other.DeviceCount && self.Devices == other.Devices
    }
}
impl ::core::cmp::Eq for SCM_LOGICAL_DEVICES {}
impl ::core::fmt::Debug for SCM_LOGICAL_DEVICES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_LOGICAL_DEVICES").field("Version", &self.Version).field("Size", &self.Size).field("DeviceCount", &self.DeviceCount).field("Devices", &self.Devices).finish()
    }
}
impl ::core::default::Default for SCM_LOGICAL_DEVICE_INSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_LOGICAL_DEVICE_INSTANCE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.DeviceGuid == other.DeviceGuid && self.SymbolicLink == other.SymbolicLink
    }
}
impl ::core::cmp::Eq for SCM_LOGICAL_DEVICE_INSTANCE {}
impl ::core::fmt::Debug for SCM_LOGICAL_DEVICE_INSTANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_LOGICAL_DEVICE_INSTANCE").field("Version", &self.Version).field("Size", &self.Size).field("DeviceGuid", &self.DeviceGuid).field("SymbolicLink", &self.SymbolicLink).finish()
    }
}
impl ::core::default::Default for SCM_PD_DESCRIPTOR_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_DESCRIPTOR_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for SCM_PD_DESCRIPTOR_HEADER {}
impl ::core::fmt::Debug for SCM_PD_DESCRIPTOR_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_DESCRIPTOR_HEADER").field("Version", &self.Version).field("Size", &self.Size).finish()
    }
}
impl ::core::default::Default for SCM_PD_DEVICE_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_DEVICE_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.DeviceGuid == other.DeviceGuid && self.DeviceHandle == other.DeviceHandle
    }
}
impl ::core::cmp::Eq for SCM_PD_DEVICE_HANDLE {}
impl ::core::fmt::Debug for SCM_PD_DEVICE_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_DEVICE_HANDLE").field("Version", &self.Version).field("Size", &self.Size).field("DeviceGuid", &self.DeviceGuid).field("DeviceHandle", &self.DeviceHandle).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCM_PD_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCM_PD_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Size == other.Size
            && self.DeviceGuid == other.DeviceGuid
            && self.UnsafeShutdownCount == other.UnsafeShutdownCount
            && self.PersistentMemorySizeInBytes == other.PersistentMemorySizeInBytes
            && self.VolatileMemorySizeInBytes == other.VolatileMemorySizeInBytes
            && self.TotalMemorySizeInBytes == other.TotalMemorySizeInBytes
            && self.SlotNumber == other.SlotNumber
            && self.DeviceHandle == other.DeviceHandle
            && self.PhysicalId == other.PhysicalId
            && self.NumberOfFormatInterfaceCodes == other.NumberOfFormatInterfaceCodes
            && self.FormatInterfaceCodes == other.FormatInterfaceCodes
            && self.VendorId == other.VendorId
            && self.ProductId == other.ProductId
            && self.SubsystemDeviceId == other.SubsystemDeviceId
            && self.SubsystemVendorId == other.SubsystemVendorId
            && self.ManufacturingLocation == other.ManufacturingLocation
            && self.ManufacturingWeek == other.ManufacturingWeek
            && self.ManufacturingYear == other.ManufacturingYear
            && self.SerialNumber4Byte == other.SerialNumber4Byte
            && self.SerialNumberLengthInChars == other.SerialNumberLengthInChars
            && self.SerialNumber == other.SerialNumber
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCM_PD_DEVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCM_PD_DEVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_DEVICE_INFO")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("DeviceGuid", &self.DeviceGuid)
            .field("UnsafeShutdownCount", &self.UnsafeShutdownCount)
            .field("PersistentMemorySizeInBytes", &self.PersistentMemorySizeInBytes)
            .field("VolatileMemorySizeInBytes", &self.VolatileMemorySizeInBytes)
            .field("TotalMemorySizeInBytes", &self.TotalMemorySizeInBytes)
            .field("SlotNumber", &self.SlotNumber)
            .field("DeviceHandle", &self.DeviceHandle)
            .field("PhysicalId", &self.PhysicalId)
            .field("NumberOfFormatInterfaceCodes", &self.NumberOfFormatInterfaceCodes)
            .field("FormatInterfaceCodes", &self.FormatInterfaceCodes)
            .field("VendorId", &self.VendorId)
            .field("ProductId", &self.ProductId)
            .field("SubsystemDeviceId", &self.SubsystemDeviceId)
            .field("SubsystemVendorId", &self.SubsystemVendorId)
            .field("ManufacturingLocation", &self.ManufacturingLocation)
            .field("ManufacturingWeek", &self.ManufacturingWeek)
            .field("ManufacturingYear", &self.ManufacturingYear)
            .field("SerialNumber4Byte", &self.SerialNumber4Byte)
            .field("SerialNumberLengthInChars", &self.SerialNumberLengthInChars)
            .field("SerialNumber", &self.SerialNumber)
            .finish()
    }
}
impl ::core::default::Default for SCM_PD_DEVICE_SPECIFIC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_DEVICE_SPECIFIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.NumberOfProperties == other.NumberOfProperties && self.DeviceSpecificProperties == other.DeviceSpecificProperties
    }
}
impl ::core::cmp::Eq for SCM_PD_DEVICE_SPECIFIC_INFO {}
impl ::core::fmt::Debug for SCM_PD_DEVICE_SPECIFIC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_DEVICE_SPECIFIC_INFO").field("Version", &self.Version).field("Size", &self.Size).field("NumberOfProperties", &self.NumberOfProperties).field("DeviceSpecificProperties", &self.DeviceSpecificProperties).finish()
    }
}
impl ::core::default::Default for SCM_PD_DEVICE_SPECIFIC_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_DEVICE_SPECIFIC_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for SCM_PD_DEVICE_SPECIFIC_PROPERTY {}
impl ::core::fmt::Debug for SCM_PD_DEVICE_SPECIFIC_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_DEVICE_SPECIFIC_PROPERTY").field("Name", &self.Name).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for SCM_PD_FIRMWARE_ACTIVATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_FIRMWARE_ACTIVATE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Flags == other.Flags && self.Slot == other.Slot
    }
}
impl ::core::cmp::Eq for SCM_PD_FIRMWARE_ACTIVATE {}
impl ::core::fmt::Debug for SCM_PD_FIRMWARE_ACTIVATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_FIRMWARE_ACTIVATE").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("Slot", &self.Slot).finish()
    }
}
impl ::core::default::Default for SCM_PD_FIRMWARE_ACTIVATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCM_PD_FIRMWARE_ACTIVATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_PD_FIRMWARE_ACTIVATION_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCM_PD_FIRMWARE_DOWNLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_FIRMWARE_DOWNLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Flags == other.Flags && self.Slot == other.Slot && self.Reserved == other.Reserved && self.Offset == other.Offset && self.FirmwareImageSizeInBytes == other.FirmwareImageSizeInBytes && self.FirmwareImage == other.FirmwareImage
    }
}
impl ::core::cmp::Eq for SCM_PD_FIRMWARE_DOWNLOAD {}
impl ::core::fmt::Debug for SCM_PD_FIRMWARE_DOWNLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_FIRMWARE_DOWNLOAD").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("Slot", &self.Slot).field("Reserved", &self.Reserved).field("Offset", &self.Offset).field("FirmwareImageSizeInBytes", &self.FirmwareImageSizeInBytes).field("FirmwareImage", &self.FirmwareImage).finish()
    }
}
impl ::core::default::Default for SCM_PD_FIRMWARE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_FIRMWARE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.ActiveSlot == other.ActiveSlot && self.NextActiveSlot == other.NextActiveSlot && self.SlotCount == other.SlotCount && self.Slots == other.Slots
    }
}
impl ::core::cmp::Eq for SCM_PD_FIRMWARE_INFO {}
impl ::core::fmt::Debug for SCM_PD_FIRMWARE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_FIRMWARE_INFO").field("Version", &self.Version).field("Size", &self.Size).field("ActiveSlot", &self.ActiveSlot).field("NextActiveSlot", &self.NextActiveSlot).field("SlotCount", &self.SlotCount).field("Slots", &self.Slots).finish()
    }
}
impl ::core::default::Default for SCM_PD_FIRMWARE_SLOT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_FIRMWARE_SLOT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.SlotNumber == other.SlotNumber && self._bitfield == other._bitfield && self.Reserved1 == other.Reserved1 && self.Revision == other.Revision
    }
}
impl ::core::cmp::Eq for SCM_PD_FIRMWARE_SLOT_INFO {}
impl ::core::fmt::Debug for SCM_PD_FIRMWARE_SLOT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_FIRMWARE_SLOT_INFO").field("Version", &self.Version).field("Size", &self.Size).field("SlotNumber", &self.SlotNumber).field("_bitfield", &self._bitfield).field("Reserved1", &self.Reserved1).field("Revision", &self.Revision).finish()
    }
}
impl ::core::default::Default for SCM_PD_FRU_ID_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_FRU_ID_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.IdentifierSize == other.IdentifierSize && self.Identifier == other.Identifier
    }
}
impl ::core::cmp::Eq for SCM_PD_FRU_ID_STRING {}
impl ::core::fmt::Debug for SCM_PD_FRU_ID_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_FRU_ID_STRING").field("Version", &self.Version).field("Size", &self.Size).field("IdentifierSize", &self.IdentifierSize).field("Identifier", &self.Identifier).finish()
    }
}
impl ::core::default::Default for SCM_PD_HEALTH_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_HEALTH_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceGuid == other.DeviceGuid
    }
}
impl ::core::cmp::Eq for SCM_PD_HEALTH_NOTIFICATION_DATA {}
impl ::core::fmt::Debug for SCM_PD_HEALTH_NOTIFICATION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_HEALTH_NOTIFICATION_DATA").field("DeviceGuid", &self.DeviceGuid).finish()
    }
}
impl ::core::default::Default for SCM_PD_HEALTH_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCM_PD_HEALTH_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_PD_HEALTH_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCM_PD_LAST_FW_ACTIVATION_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCM_PD_LAST_FW_ACTIVATION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_PD_LAST_FW_ACTIVATION_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCM_PD_LOCATION_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_LOCATION_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Location == other.Location
    }
}
impl ::core::cmp::Eq for SCM_PD_LOCATION_STRING {}
impl ::core::fmt::Debug for SCM_PD_LOCATION_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_LOCATION_STRING").field("Version", &self.Version).field("Size", &self.Size).field("Location", &self.Location).finish()
    }
}
impl ::core::default::Default for SCM_PD_MANAGEMENT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_MANAGEMENT_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Health == other.Health && self.NumberOfOperationalStatus == other.NumberOfOperationalStatus && self.NumberOfAdditionalReasons == other.NumberOfAdditionalReasons && self.OperationalStatus == other.OperationalStatus && self.AdditionalReasons == other.AdditionalReasons
    }
}
impl ::core::cmp::Eq for SCM_PD_MANAGEMENT_STATUS {}
impl ::core::fmt::Debug for SCM_PD_MANAGEMENT_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_MANAGEMENT_STATUS").field("Version", &self.Version).field("Size", &self.Size).field("Health", &self.Health).field("NumberOfOperationalStatus", &self.NumberOfOperationalStatus).field("NumberOfAdditionalReasons", &self.NumberOfAdditionalReasons).field("OperationalStatus", &self.OperationalStatus).field("AdditionalReasons", &self.AdditionalReasons).finish()
    }
}
impl ::core::default::Default for SCM_PD_MEDIA_REINITIALIZATION_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCM_PD_MEDIA_REINITIALIZATION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_PD_MEDIA_REINITIALIZATION_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCM_PD_OPERATIONAL_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCM_PD_OPERATIONAL_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_PD_OPERATIONAL_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCM_PD_OPERATIONAL_STATUS_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCM_PD_OPERATIONAL_STATUS_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_PD_OPERATIONAL_STATUS_REASON").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCM_PD_PASSTHROUGH_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_PASSTHROUGH_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.ProtocolGuid == other.ProtocolGuid && self.DataSize == other.DataSize && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for SCM_PD_PASSTHROUGH_INPUT {}
impl ::core::fmt::Debug for SCM_PD_PASSTHROUGH_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_PASSTHROUGH_INPUT").field("Version", &self.Version).field("Size", &self.Size).field("ProtocolGuid", &self.ProtocolGuid).field("DataSize", &self.DataSize).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for SCM_PD_PASSTHROUGH_INVDIMM_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_PASSTHROUGH_INVDIMM_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Opcode == other.Opcode && self.OpcodeParametersLength == other.OpcodeParametersLength && self.OpcodeParameters == other.OpcodeParameters
    }
}
impl ::core::cmp::Eq for SCM_PD_PASSTHROUGH_INVDIMM_INPUT {}
impl ::core::fmt::Debug for SCM_PD_PASSTHROUGH_INVDIMM_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_PASSTHROUGH_INVDIMM_INPUT").field("Opcode", &self.Opcode).field("OpcodeParametersLength", &self.OpcodeParametersLength).field("OpcodeParameters", &self.OpcodeParameters).finish()
    }
}
impl ::core::default::Default for SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.GeneralStatus == other.GeneralStatus && self.ExtendedStatus == other.ExtendedStatus && self.OutputDataLength == other.OutputDataLength && self.OutputData == other.OutputData
    }
}
impl ::core::cmp::Eq for SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT {}
impl ::core::fmt::Debug for SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_PASSTHROUGH_INVDIMM_OUTPUT").field("GeneralStatus", &self.GeneralStatus).field("ExtendedStatus", &self.ExtendedStatus).field("OutputDataLength", &self.OutputDataLength).field("OutputData", &self.OutputData).finish()
    }
}
impl ::core::default::Default for SCM_PD_PASSTHROUGH_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_PASSTHROUGH_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.ProtocolGuid == other.ProtocolGuid && self.DataSize == other.DataSize && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for SCM_PD_PASSTHROUGH_OUTPUT {}
impl ::core::fmt::Debug for SCM_PD_PASSTHROUGH_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_PASSTHROUGH_OUTPUT").field("Version", &self.Version).field("Size", &self.Size).field("ProtocolGuid", &self.ProtocolGuid).field("DataSize", &self.DataSize).field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for SCM_PD_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCM_PD_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_PD_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCM_PD_PROPERTY_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_PROPERTY_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.PropertyId == other.PropertyId && self.QueryType == other.QueryType && self.AdditionalParameters == other.AdditionalParameters
    }
}
impl ::core::cmp::Eq for SCM_PD_PROPERTY_QUERY {}
impl ::core::fmt::Debug for SCM_PD_PROPERTY_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_PROPERTY_QUERY").field("Version", &self.Version).field("Size", &self.Size).field("PropertyId", &self.PropertyId).field("QueryType", &self.QueryType).field("AdditionalParameters", &self.AdditionalParameters).finish()
    }
}
impl ::core::default::Default for SCM_PD_PROPERTY_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_PROPERTY_SET {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.PropertyId == other.PropertyId && self.SetType == other.SetType && self.AdditionalParameters == other.AdditionalParameters
    }
}
impl ::core::cmp::Eq for SCM_PD_PROPERTY_SET {}
impl ::core::fmt::Debug for SCM_PD_PROPERTY_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_PROPERTY_SET").field("Version", &self.Version).field("Size", &self.Size).field("PropertyId", &self.PropertyId).field("SetType", &self.SetType).field("AdditionalParameters", &self.AdditionalParameters).finish()
    }
}
impl ::core::default::Default for SCM_PD_QUERY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCM_PD_QUERY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_PD_QUERY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCM_PD_REINITIALIZE_MEDIA_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_REINITIALIZE_MEDIA_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Options == other.Options
    }
}
impl ::core::cmp::Eq for SCM_PD_REINITIALIZE_MEDIA_INPUT {}
impl ::core::fmt::Debug for SCM_PD_REINITIALIZE_MEDIA_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_REINITIALIZE_MEDIA_INPUT").field("Version", &self.Version).field("Size", &self.Size).field("Options", &self.Options).finish()
    }
}
impl ::core::default::Default for SCM_PD_REINITIALIZE_MEDIA_INPUT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_REINITIALIZE_MEDIA_INPUT_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for SCM_PD_REINITIALIZE_MEDIA_INPUT_0 {}
impl ::core::fmt::Debug for SCM_PD_REINITIALIZE_MEDIA_INPUT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_REINITIALIZE_MEDIA_INPUT_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for SCM_PD_REINITIALIZE_MEDIA_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_REINITIALIZE_MEDIA_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Status == other.Status
    }
}
impl ::core::cmp::Eq for SCM_PD_REINITIALIZE_MEDIA_OUTPUT {}
impl ::core::fmt::Debug for SCM_PD_REINITIALIZE_MEDIA_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_REINITIALIZE_MEDIA_OUTPUT").field("Version", &self.Version).field("Size", &self.Size).field("Status", &self.Status).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.ArmState == other.ArmState
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_RUNTIME_FW_ACTIVATION_ARM_STATE").field("ArmState", &self.ArmState).finish()
    }
}
impl ::core::default::Default for SCM_PD_RUNTIME_FW_ACTIVATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PD_RUNTIME_FW_ACTIVATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.LastFirmwareActivationStatus == other.LastFirmwareActivationStatus && self.FirmwareActivationState == other.FirmwareActivationState
    }
}
impl ::core::cmp::Eq for SCM_PD_RUNTIME_FW_ACTIVATION_INFO {}
impl ::core::fmt::Debug for SCM_PD_RUNTIME_FW_ACTIVATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PD_RUNTIME_FW_ACTIVATION_INFO").field("Version", &self.Version).field("Size", &self.Size).field("LastFirmwareActivationStatus", &self.LastFirmwareActivationStatus).field("FirmwareActivationState", &self.FirmwareActivationState).finish()
    }
}
impl ::core::default::Default for SCM_PD_SET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCM_PD_SET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_PD_SET_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for SCM_PHYSICAL_DEVICES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PHYSICAL_DEVICES {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.DeviceCount == other.DeviceCount && self.Devices == other.Devices
    }
}
impl ::core::cmp::Eq for SCM_PHYSICAL_DEVICES {}
impl ::core::fmt::Debug for SCM_PHYSICAL_DEVICES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PHYSICAL_DEVICES").field("Version", &self.Version).field("Size", &self.Size).field("DeviceCount", &self.DeviceCount).field("Devices", &self.Devices).finish()
    }
}
impl ::core::default::Default for SCM_PHYSICAL_DEVICE_INSTANCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_PHYSICAL_DEVICE_INSTANCE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.NfitHandle == other.NfitHandle && self.SymbolicLink == other.SymbolicLink
    }
}
impl ::core::cmp::Eq for SCM_PHYSICAL_DEVICE_INSTANCE {}
impl ::core::fmt::Debug for SCM_PHYSICAL_DEVICE_INSTANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_PHYSICAL_DEVICE_INSTANCE").field("Version", &self.Version).field("Size", &self.Size).field("NfitHandle", &self.NfitHandle).field("SymbolicLink", &self.SymbolicLink).finish()
    }
}
impl ::core::default::Default for SCM_REGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_REGION {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Flags == other.Flags && self.NfitHandle == other.NfitHandle && self.LogicalDeviceGuid == other.LogicalDeviceGuid && self.AddressRangeType == other.AddressRangeType && self.AssociatedId == other.AssociatedId && self.Length == other.Length && self.StartingDPA == other.StartingDPA && self.BaseSPA == other.BaseSPA && self.SPAOffset == other.SPAOffset && self.RegionOffset == other.RegionOffset
    }
}
impl ::core::cmp::Eq for SCM_REGION {}
impl ::core::fmt::Debug for SCM_REGION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_REGION")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("Flags", &self.Flags)
            .field("NfitHandle", &self.NfitHandle)
            .field("LogicalDeviceGuid", &self.LogicalDeviceGuid)
            .field("AddressRangeType", &self.AddressRangeType)
            .field("AssociatedId", &self.AssociatedId)
            .field("Length", &self.Length)
            .field("StartingDPA", &self.StartingDPA)
            .field("BaseSPA", &self.BaseSPA)
            .field("SPAOffset", &self.SPAOffset)
            .field("RegionOffset", &self.RegionOffset)
            .finish()
    }
}
impl ::core::default::Default for SCM_REGIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SCM_REGIONS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.RegionCount == other.RegionCount && self.Regions == other.Regions
    }
}
impl ::core::cmp::Eq for SCM_REGIONS {}
impl ::core::fmt::Debug for SCM_REGIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCM_REGIONS").field("Version", &self.Version).field("Size", &self.Size).field("RegionCount", &self.RegionCount).field("Regions", &self.Regions).finish()
    }
}
impl ::core::default::Default for SCM_REGION_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SCM_REGION_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SCM_REGION_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for SD_CHANGE_MACHINE_SID_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SD_CHANGE_MACHINE_SID_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentMachineSIDOffset == other.CurrentMachineSIDOffset && self.CurrentMachineSIDLength == other.CurrentMachineSIDLength && self.NewMachineSIDOffset == other.NewMachineSIDOffset && self.NewMachineSIDLength == other.NewMachineSIDLength
    }
}
impl ::core::cmp::Eq for SD_CHANGE_MACHINE_SID_INPUT {}
impl ::core::fmt::Debug for SD_CHANGE_MACHINE_SID_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SD_CHANGE_MACHINE_SID_INPUT").field("CurrentMachineSIDOffset", &self.CurrentMachineSIDOffset).field("CurrentMachineSIDLength", &self.CurrentMachineSIDLength).field("NewMachineSIDOffset", &self.NewMachineSIDOffset).field("NewMachineSIDLength", &self.NewMachineSIDLength).finish()
    }
}
impl ::core::default::Default for SD_CHANGE_MACHINE_SID_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SD_CHANGE_MACHINE_SID_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.NumSDChangedSuccess == other.NumSDChangedSuccess && self.NumSDChangedFail == other.NumSDChangedFail && self.NumSDUnused == other.NumSDUnused && self.NumSDTotal == other.NumSDTotal && self.NumMftSDChangedSuccess == other.NumMftSDChangedSuccess && self.NumMftSDChangedFail == other.NumMftSDChangedFail && self.NumMftSDTotal == other.NumMftSDTotal
    }
}
impl ::core::cmp::Eq for SD_CHANGE_MACHINE_SID_OUTPUT {}
impl ::core::fmt::Debug for SD_CHANGE_MACHINE_SID_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SD_CHANGE_MACHINE_SID_OUTPUT").field("NumSDChangedSuccess", &self.NumSDChangedSuccess).field("NumSDChangedFail", &self.NumSDChangedFail).field("NumSDUnused", &self.NumSDUnused).field("NumSDTotal", &self.NumSDTotal).field("NumMftSDChangedSuccess", &self.NumMftSDChangedSuccess).field("NumMftSDChangedFail", &self.NumMftSDChangedFail).field("NumMftSDTotal", &self.NumMftSDTotal).finish()
    }
}
impl ::core::default::Default for SD_ENUM_SDS_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SD_ENUM_SDS_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Hash == other.Hash && self.SecurityId == other.SecurityId && self.Offset == other.Offset && self.Length == other.Length && self.Descriptor == other.Descriptor
    }
}
impl ::core::cmp::Eq for SD_ENUM_SDS_ENTRY {}
impl ::core::fmt::Debug for SD_ENUM_SDS_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SD_ENUM_SDS_ENTRY").field("Hash", &self.Hash).field("SecurityId", &self.SecurityId).field("Offset", &self.Offset).field("Length", &self.Length).field("Descriptor", &self.Descriptor).finish()
    }
}
impl ::core::default::Default for SD_ENUM_SDS_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SD_ENUM_SDS_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.StartingOffset == other.StartingOffset && self.MaxSDEntriesToReturn == other.MaxSDEntriesToReturn
    }
}
impl ::core::cmp::Eq for SD_ENUM_SDS_INPUT {}
impl ::core::fmt::Debug for SD_ENUM_SDS_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SD_ENUM_SDS_INPUT").field("StartingOffset", &self.StartingOffset).field("MaxSDEntriesToReturn", &self.MaxSDEntriesToReturn).finish()
    }
}
impl ::core::default::Default for SD_ENUM_SDS_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SD_ENUM_SDS_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.NextOffset == other.NextOffset && self.NumSDEntriesReturned == other.NumSDEntriesReturned && self.NumSDBytesReturned == other.NumSDBytesReturned && self.SDEntry == other.SDEntry
    }
}
impl ::core::cmp::Eq for SD_ENUM_SDS_OUTPUT {}
impl ::core::fmt::Debug for SD_ENUM_SDS_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SD_ENUM_SDS_OUTPUT").field("NextOffset", &self.NextOffset).field("NumSDEntriesReturned", &self.NumSDEntriesReturned).field("NumSDBytesReturned", &self.NumSDBytesReturned).field("SDEntry", &self.SDEntry).finish()
    }
}
impl ::core::default::Default for SD_GLOBAL_CHANGE_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SD_GLOBAL_CHANGE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SD_QUERY_STATS_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SD_QUERY_STATS_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for SD_QUERY_STATS_INPUT {}
impl ::core::fmt::Debug for SD_QUERY_STATS_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SD_QUERY_STATS_INPUT").field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for SD_QUERY_STATS_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SD_QUERY_STATS_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.SdsStreamSize == other.SdsStreamSize && self.SdsAllocationSize == other.SdsAllocationSize && self.SiiStreamSize == other.SiiStreamSize && self.SiiAllocationSize == other.SiiAllocationSize && self.SdhStreamSize == other.SdhStreamSize && self.SdhAllocationSize == other.SdhAllocationSize && self.NumSDTotal == other.NumSDTotal && self.NumSDUnused == other.NumSDUnused
    }
}
impl ::core::cmp::Eq for SD_QUERY_STATS_OUTPUT {}
impl ::core::fmt::Debug for SD_QUERY_STATS_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SD_QUERY_STATS_OUTPUT").field("SdsStreamSize", &self.SdsStreamSize).field("SdsAllocationSize", &self.SdsAllocationSize).field("SiiStreamSize", &self.SiiStreamSize).field("SiiAllocationSize", &self.SiiAllocationSize).field("SdhStreamSize", &self.SdhStreamSize).field("SdhAllocationSize", &self.SdhAllocationSize).field("NumSDTotal", &self.NumSDTotal).field("NumSDUnused", &self.NumSDUnused).finish()
    }
}
impl ::core::default::Default for SENDCMDINPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SENDCMDOUTPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.AlignmentShift == other.AlignmentShift && self.FileOffsetToAlign == other.FileOffsetToAlign && self.FallbackAlignmentShift == other.FallbackAlignmentShift
    }
}
impl ::core::cmp::Eq for SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT {}
impl ::core::fmt::Debug for SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SET_DAX_ALLOC_ALIGNMENT_HINT_INPUT").field("Flags", &self.Flags).field("AlignmentShift", &self.AlignmentShift).field("FileOffsetToAlign", &self.FileOffsetToAlign).field("FallbackAlignmentShift", &self.FallbackAlignmentShift).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SET_DISK_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SET_DISK_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Persist == other.Persist && self.Reserved1 == other.Reserved1 && self.Attributes == other.Attributes && self.AttributesMask == other.AttributesMask && self.Reserved2 == other.Reserved2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SET_DISK_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SET_DISK_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SET_DISK_ATTRIBUTES").field("Version", &self.Version).field("Persist", &self.Persist).field("Reserved1", &self.Reserved1).field("Attributes", &self.Attributes).field("AttributesMask", &self.AttributesMask).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::core::default::Default for SET_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SET_PARTITION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.PartitionType == other.PartitionType
    }
}
impl ::core::cmp::Eq for SET_PARTITION_INFORMATION {}
impl ::core::fmt::Debug for SET_PARTITION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SET_PARTITION_INFORMATION").field("PartitionType", &self.PartitionType).finish()
    }
}
impl ::core::default::Default for SET_PARTITION_INFORMATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SET_PURGE_FAILURE_MODE_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SET_PURGE_FAILURE_MODE_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for SET_PURGE_FAILURE_MODE_INPUT {}
impl ::core::fmt::Debug for SET_PURGE_FAILURE_MODE_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SET_PURGE_FAILURE_MODE_INPUT").field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for SHRINK_VOLUME_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SHRINK_VOLUME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ShrinkRequestType == other.ShrinkRequestType && self.Flags == other.Flags && self.NewNumberOfSectors == other.NewNumberOfSectors
    }
}
impl ::core::cmp::Eq for SHRINK_VOLUME_INFORMATION {}
impl ::core::fmt::Debug for SHRINK_VOLUME_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SHRINK_VOLUME_INFORMATION").field("ShrinkRequestType", &self.ShrinkRequestType).field("Flags", &self.Flags).field("NewNumberOfSectors", &self.NewNumberOfSectors).finish()
    }
}
impl ::core::default::Default for SHRINK_VOLUME_REQUEST_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for SHRINK_VOLUME_REQUEST_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHRINK_VOLUME_REQUEST_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for SI_COPYFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SI_COPYFILE {
    fn eq(&self, other: &Self) -> bool {
        self.SourceFileNameLength == other.SourceFileNameLength && self.DestinationFileNameLength == other.DestinationFileNameLength && self.Flags == other.Flags && self.FileNameBuffer == other.FileNameBuffer
    }
}
impl ::core::cmp::Eq for SI_COPYFILE {}
impl ::core::fmt::Debug for SI_COPYFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SI_COPYFILE").field("SourceFileNameLength", &self.SourceFileNameLength).field("DestinationFileNameLength", &self.DestinationFileNameLength).field("Flags", &self.Flags).field("FileNameBuffer", &self.FileNameBuffer).finish()
    }
}
impl ::core::default::Default for SMB_SHARE_FLUSH_AND_PURGE_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SMB_SHARE_FLUSH_AND_PURGE_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
    }
}
impl ::core::cmp::Eq for SMB_SHARE_FLUSH_AND_PURGE_INPUT {}
impl ::core::fmt::Debug for SMB_SHARE_FLUSH_AND_PURGE_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMB_SHARE_FLUSH_AND_PURGE_INPUT").field("Version", &self.Version).finish()
    }
}
impl ::core::default::Default for SMB_SHARE_FLUSH_AND_PURGE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SMB_SHARE_FLUSH_AND_PURGE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.cEntriesPurged == other.cEntriesPurged
    }
}
impl ::core::cmp::Eq for SMB_SHARE_FLUSH_AND_PURGE_OUTPUT {}
impl ::core::fmt::Debug for SMB_SHARE_FLUSH_AND_PURGE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMB_SHARE_FLUSH_AND_PURGE_OUTPUT").field("cEntriesPurged", &self.cEntriesPurged).finish()
    }
}
impl ::core::default::Default for STARTING_LCN_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STARTING_LCN_INPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.StartingLcn == other.StartingLcn
    }
}
impl ::core::cmp::Eq for STARTING_LCN_INPUT_BUFFER {}
impl ::core::fmt::Debug for STARTING_LCN_INPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTING_LCN_INPUT_BUFFER").field("StartingLcn", &self.StartingLcn).finish()
    }
}
impl ::core::default::Default for STARTING_LCN_INPUT_BUFFER_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STARTING_LCN_INPUT_BUFFER_EX {
    fn eq(&self, other: &Self) -> bool {
        self.StartingLcn == other.StartingLcn && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for STARTING_LCN_INPUT_BUFFER_EX {}
impl ::core::fmt::Debug for STARTING_LCN_INPUT_BUFFER_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTING_LCN_INPUT_BUFFER_EX").field("StartingLcn", &self.StartingLcn).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for STARTING_VCN_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STARTING_VCN_INPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.StartingVcn == other.StartingVcn
    }
}
impl ::core::cmp::Eq for STARTING_VCN_INPUT_BUFFER {}
impl ::core::fmt::Debug for STARTING_VCN_INPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTING_VCN_INPUT_BUFFER").field("StartingVcn", &self.StartingVcn).finish()
    }
}
impl ::core::default::Default for STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.BytesPerCacheLine == other.BytesPerCacheLine && self.BytesOffsetForCacheAlignment == other.BytesOffsetForCacheAlignment && self.BytesPerLogicalSector == other.BytesPerLogicalSector && self.BytesPerPhysicalSector == other.BytesPerPhysicalSector && self.BytesOffsetForSectorAlignment == other.BytesOffsetForSectorAlignment
    }
}
impl ::core::cmp::Eq for STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR {}
impl ::core::fmt::Debug for STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ACCESS_ALIGNMENT_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("BytesPerCacheLine", &self.BytesPerCacheLine).field("BytesOffsetForCacheAlignment", &self.BytesOffsetForCacheAlignment).field("BytesPerLogicalSector", &self.BytesPerLogicalSector).field("BytesPerPhysicalSector", &self.BytesPerPhysicalSector).field("BytesOffsetForSectorAlignment", &self.BytesOffsetForSectorAlignment).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_ADAPTER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_ADAPTER_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.MaximumTransferLength == other.MaximumTransferLength && self.MaximumPhysicalPages == other.MaximumPhysicalPages && self.AlignmentMask == other.AlignmentMask && self.AdapterUsesPio == other.AdapterUsesPio && self.AdapterScansDown == other.AdapterScansDown && self.CommandQueueing == other.CommandQueueing && self.AcceleratedTransfer == other.AcceleratedTransfer && self.BusType == other.BusType && self.BusMajorVersion == other.BusMajorVersion && self.BusMinorVersion == other.BusMinorVersion && self.SrbType == other.SrbType && self.AddressType == other.AddressType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_ADAPTER_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_ADAPTER_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ADAPTER_DESCRIPTOR")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("MaximumTransferLength", &self.MaximumTransferLength)
            .field("MaximumPhysicalPages", &self.MaximumPhysicalPages)
            .field("AlignmentMask", &self.AlignmentMask)
            .field("AdapterUsesPio", &self.AdapterUsesPio)
            .field("AdapterScansDown", &self.AdapterScansDown)
            .field("CommandQueueing", &self.CommandQueueing)
            .field("AcceleratedTransfer", &self.AcceleratedTransfer)
            .field("BusType", &self.BusType)
            .field("BusMajorVersion", &self.BusMajorVersion)
            .field("BusMinorVersion", &self.BusMinorVersion)
            .field("SrbType", &self.SrbType)
            .field("AddressType", &self.AddressType)
            .finish()
    }
}
impl ::core::default::Default for STORAGE_ADAPTER_SERIAL_NUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_ADAPTER_SERIAL_NUMBER {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.SerialNumber == other.SerialNumber
    }
}
impl ::core::cmp::Eq for STORAGE_ADAPTER_SERIAL_NUMBER {}
impl ::core::fmt::Debug for STORAGE_ADAPTER_SERIAL_NUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ADAPTER_SERIAL_NUMBER").field("Version", &self.Version).field("Size", &self.Size).field("SerialNumber", &self.SerialNumber).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_ALLOCATE_BC_STREAM_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_ALLOCATE_BC_STREAM_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.RequestsPerPeriod == other.RequestsPerPeriod && self.Period == other.Period && self.RetryFailures == other.RetryFailures && self.Discardable == other.Discardable && self.Reserved1 == other.Reserved1 && self.AccessType == other.AccessType && self.AccessMode == other.AccessMode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_ALLOCATE_BC_STREAM_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_ALLOCATE_BC_STREAM_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ALLOCATE_BC_STREAM_INPUT").field("Version", &self.Version).field("RequestsPerPeriod", &self.RequestsPerPeriod).field("Period", &self.Period).field("RetryFailures", &self.RetryFailures).field("Discardable", &self.Discardable).field("Reserved1", &self.Reserved1).field("AccessType", &self.AccessType).field("AccessMode", &self.AccessMode).finish()
    }
}
impl ::core::default::Default for STORAGE_ALLOCATE_BC_STREAM_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_ALLOCATE_BC_STREAM_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.RequestSize == other.RequestSize && self.NumOutStandingRequests == other.NumOutStandingRequests
    }
}
impl ::core::cmp::Eq for STORAGE_ALLOCATE_BC_STREAM_OUTPUT {}
impl ::core::fmt::Debug for STORAGE_ALLOCATE_BC_STREAM_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ALLOCATE_BC_STREAM_OUTPUT").field("RequestSize", &self.RequestSize).field("NumOutStandingRequests", &self.NumOutStandingRequests).finish()
    }
}
impl ::core::default::Default for STORAGE_ASSOCIATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_ASSOCIATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_ASSOCIATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_ATTRIBUTE_MGMT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_ATTRIBUTE_MGMT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Action == other.Action && self.Attribute == other.Attribute
    }
}
impl ::core::cmp::Eq for STORAGE_ATTRIBUTE_MGMT {}
impl ::core::fmt::Debug for STORAGE_ATTRIBUTE_MGMT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ATTRIBUTE_MGMT").field("Version", &self.Version).field("Size", &self.Size).field("Action", &self.Action).field("Attribute", &self.Attribute).finish()
    }
}
impl ::core::default::Default for STORAGE_ATTRIBUTE_MGMT_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_ATTRIBUTE_MGMT_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_ATTRIBUTE_MGMT_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_BREAK_RESERVATION_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_BREAK_RESERVATION_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self._unused == other._unused && self.PathId == other.PathId && self.TargetId == other.TargetId && self.Lun == other.Lun
    }
}
impl ::core::cmp::Eq for STORAGE_BREAK_RESERVATION_REQUEST {}
impl ::core::fmt::Debug for STORAGE_BREAK_RESERVATION_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_BREAK_RESERVATION_REQUEST").field("Length", &self.Length).field("_unused", &self._unused).field("PathId", &self.PathId).field("TargetId", &self.TargetId).field("Lun", &self.Lun).finish()
    }
}
impl ::core::default::Default for STORAGE_BUS_RESET_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_BUS_RESET_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.PathId == other.PathId
    }
}
impl ::core::cmp::Eq for STORAGE_BUS_RESET_REQUEST {}
impl ::core::fmt::Debug for STORAGE_BUS_RESET_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_BUS_RESET_REQUEST").field("PathId", &self.PathId).finish()
    }
}
impl ::core::default::Default for STORAGE_COMPONENT_HEALTH_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_COMPONENT_HEALTH_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_COMPONENT_HEALTH_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_COUNTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STORAGE_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STORAGE_COUNTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_COUNTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_COUNTER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_CRYPTO_ALGORITHM_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_CRYPTO_ALGORITHM_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_CRYPTO_ALGORITHM_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_CRYPTO_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_CRYPTO_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.CryptoCapabilityIndex == other.CryptoCapabilityIndex && self.AlgorithmId == other.AlgorithmId && self.KeySize == other.KeySize && self.DataUnitSizeBitmask == other.DataUnitSizeBitmask
    }
}
impl ::core::cmp::Eq for STORAGE_CRYPTO_CAPABILITY {}
impl ::core::fmt::Debug for STORAGE_CRYPTO_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_CRYPTO_CAPABILITY").field("Version", &self.Version).field("Size", &self.Size).field("CryptoCapabilityIndex", &self.CryptoCapabilityIndex).field("AlgorithmId", &self.AlgorithmId).field("KeySize", &self.KeySize).field("DataUnitSizeBitmask", &self.DataUnitSizeBitmask).finish()
    }
}
impl ::core::default::Default for STORAGE_CRYPTO_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_CRYPTO_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.NumKeysSupported == other.NumKeysSupported && self.NumCryptoCapabilities == other.NumCryptoCapabilities && self.CryptoCapabilities == other.CryptoCapabilities
    }
}
impl ::core::cmp::Eq for STORAGE_CRYPTO_DESCRIPTOR {}
impl ::core::fmt::Debug for STORAGE_CRYPTO_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_CRYPTO_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("NumKeysSupported", &self.NumKeysSupported).field("NumCryptoCapabilities", &self.NumCryptoCapabilities).field("CryptoCapabilities", &self.CryptoCapabilities).finish()
    }
}
impl ::core::default::Default for STORAGE_CRYPTO_KEY_SIZE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_CRYPTO_KEY_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_CRYPTO_KEY_SIZE").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_DESCRIPTOR_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_DESCRIPTOR_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for STORAGE_DESCRIPTOR_HEADER {}
impl ::core::fmt::Debug for STORAGE_DESCRIPTOR_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DESCRIPTOR_HEADER").field("Version", &self.Version).field("Size", &self.Size).finish()
    }
}
impl ::core::default::Default for STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Attributes == other.Attributes
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR {}
impl ::core::fmt::Debug for STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_ATTRIBUTES_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("Attributes", &self.Attributes).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::default::Default for STORAGE_DEVICE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::PartialEq for STORAGE_DEVICE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.DeviceType == other.DeviceType && self.DeviceTypeModifier == other.DeviceTypeModifier && self.RemovableMedia == other.RemovableMedia && self.CommandQueueing == other.CommandQueueing && self.VendorIdOffset == other.VendorIdOffset && self.ProductIdOffset == other.ProductIdOffset && self.ProductRevisionOffset == other.ProductRevisionOffset && self.SerialNumberOffset == other.SerialNumberOffset && self.BusType == other.BusType && self.RawPropertiesLength == other.RawPropertiesLength && self.RawDeviceProperties == other.RawDeviceProperties
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::cmp::Eq for STORAGE_DEVICE_DESCRIPTOR {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem"))]
impl ::core::fmt::Debug for STORAGE_DEVICE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_DESCRIPTOR")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("DeviceType", &self.DeviceType)
            .field("DeviceTypeModifier", &self.DeviceTypeModifier)
            .field("RemovableMedia", &self.RemovableMedia)
            .field("CommandQueueing", &self.CommandQueueing)
            .field("VendorIdOffset", &self.VendorIdOffset)
            .field("ProductIdOffset", &self.ProductIdOffset)
            .field("ProductRevisionOffset", &self.ProductRevisionOffset)
            .field("SerialNumberOffset", &self.SerialNumberOffset)
            .field("BusType", &self.BusType)
            .field("RawPropertiesLength", &self.RawPropertiesLength)
            .field("RawDeviceProperties", &self.RawDeviceProperties)
            .finish()
    }
}
impl ::core::default::Default for STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.NumberOfFaultDomains == other.NumberOfFaultDomains && self.FaultDomainIds == other.FaultDomainIds
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {}
impl ::core::fmt::Debug for STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_FAULT_DOMAIN_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("NumberOfFaultDomains", &self.NumberOfFaultDomains).field("FaultDomainIds", &self.FaultDomainIds).finish()
    }
}
impl ::core::default::Default for STORAGE_DEVICE_FORM_FACTOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_DEVICE_FORM_FACTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_DEVICE_FORM_FACTOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_DEVICE_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_ID_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.NumberOfIdentifiers == other.NumberOfIdentifiers && self.Identifiers == other.Identifiers
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_ID_DESCRIPTOR {}
impl ::core::fmt::Debug for STORAGE_DEVICE_ID_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_ID_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("NumberOfIdentifiers", &self.NumberOfIdentifiers).field("Identifiers", &self.Identifiers).finish()
    }
}
impl ::core::default::Default for STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.LunMaxIoCount == other.LunMaxIoCount && self.AdapterMaxIoCount == other.AdapterMaxIoCount
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR {}
impl ::core::fmt::Debug for STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_IO_CAPABILITY_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("LunMaxIoCount", &self.LunMaxIoCount).field("AdapterMaxIoCount", &self.AdapterMaxIoCount).finish()
    }
}
impl ::core::default::Default for STORAGE_DEVICE_LED_STATE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_LED_STATE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.State == other.State
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_LED_STATE_DESCRIPTOR {}
impl ::core::fmt::Debug for STORAGE_DEVICE_LED_STATE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_LED_STATE_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("State", &self.State).finish()
    }
}
impl ::core::default::Default for STORAGE_DEVICE_LOCATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STORAGE_DEVICE_MANAGEMENT_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STORAGE_DEVICE_NUMA_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_NUMA_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.NumaNode == other.NumaNode
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_NUMA_PROPERTY {}
impl ::core::fmt::Debug for STORAGE_DEVICE_NUMA_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_NUMA_PROPERTY").field("Version", &self.Version).field("Size", &self.Size).field("NumaNode", &self.NumaNode).finish()
    }
}
impl ::core::default::Default for STORAGE_DEVICE_NUMBER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_NUMBER {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceType == other.DeviceType && self.DeviceNumber == other.DeviceNumber && self.PartitionNumber == other.PartitionNumber
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_NUMBER {}
impl ::core::fmt::Debug for STORAGE_DEVICE_NUMBER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_NUMBER").field("DeviceType", &self.DeviceType).field("DeviceNumber", &self.DeviceNumber).field("PartitionNumber", &self.PartitionNumber).finish()
    }
}
impl ::core::default::Default for STORAGE_DEVICE_NUMBERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_NUMBERS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.NumberOfDevices == other.NumberOfDevices && self.Devices == other.Devices
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_NUMBERS {}
impl ::core::fmt::Debug for STORAGE_DEVICE_NUMBERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_NUMBERS").field("Version", &self.Version).field("Size", &self.Size).field("NumberOfDevices", &self.NumberOfDevices).field("Devices", &self.Devices).finish()
    }
}
impl ::core::default::Default for STORAGE_DEVICE_NUMBER_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_NUMBER_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Flags == other.Flags && self.DeviceType == other.DeviceType && self.DeviceNumber == other.DeviceNumber && self.DeviceGuid == other.DeviceGuid && self.PartitionNumber == other.PartitionNumber
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_NUMBER_EX {}
impl ::core::fmt::Debug for STORAGE_DEVICE_NUMBER_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_NUMBER_EX").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("DeviceType", &self.DeviceType).field("DeviceNumber", &self.DeviceNumber).field("DeviceGuid", &self.DeviceGuid).field("PartitionNumber", &self.PartitionNumber).finish()
    }
}
impl ::core::default::Default for STORAGE_DEVICE_POWER_CAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_POWER_CAP {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Units == other.Units && self.MaxPower == other.MaxPower
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_POWER_CAP {}
impl ::core::fmt::Debug for STORAGE_DEVICE_POWER_CAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_POWER_CAP").field("Version", &self.Version).field("Size", &self.Size).field("Units", &self.Units).field("MaxPower", &self.MaxPower).finish()
    }
}
impl ::core::default::Default for STORAGE_DEVICE_POWER_CAP_UNITS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_DEVICE_POWER_CAP_UNITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_DEVICE_POWER_CAP_UNITS").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_DEVICE_RESILIENCY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_RESILIENCY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.NameOffset == other.NameOffset && self.NumberOfLogicalCopies == other.NumberOfLogicalCopies && self.NumberOfPhysicalCopies == other.NumberOfPhysicalCopies && self.PhysicalDiskRedundancy == other.PhysicalDiskRedundancy && self.NumberOfColumns == other.NumberOfColumns && self.Interleave == other.Interleave
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_RESILIENCY_DESCRIPTOR {}
impl ::core::fmt::Debug for STORAGE_DEVICE_RESILIENCY_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_RESILIENCY_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("NameOffset", &self.NameOffset).field("NumberOfLogicalCopies", &self.NumberOfLogicalCopies).field("NumberOfPhysicalCopies", &self.NumberOfPhysicalCopies).field("PhysicalDiskRedundancy", &self.PhysicalDiskRedundancy).field("NumberOfColumns", &self.NumberOfColumns).field("Interleave", &self.Interleave).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.SupportsSelfEncryption == other.SupportsSelfEncryption
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_SELF_ENCRYPTION_PROPERTY").field("Version", &self.Version).field("Size", &self.Size).field("SupportsSelfEncryption", &self.SupportsSelfEncryption).finish()
    }
}
impl ::core::default::Default for STORAGE_DEVICE_TIERING_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_TIERING_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Flags == other.Flags && self.TotalNumberOfTiers == other.TotalNumberOfTiers && self.NumberOfTiersReturned == other.NumberOfTiersReturned && self.Tiers == other.Tiers
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_TIERING_DESCRIPTOR {}
impl ::core::fmt::Debug for STORAGE_DEVICE_TIERING_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_TIERING_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("TotalNumberOfTiers", &self.TotalNumberOfTiers).field("NumberOfTiersReturned", &self.NumberOfTiersReturned).field("Tiers", &self.Tiers).finish()
    }
}
impl ::core::default::Default for STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.UnsafeShutdownCount == other.UnsafeShutdownCount
    }
}
impl ::core::cmp::Eq for STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT {}
impl ::core::fmt::Debug for STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DEVICE_UNSAFE_SHUTDOWN_COUNT").field("Version", &self.Version).field("Size", &self.Size).field("UnsafeShutdownCount", &self.UnsafeShutdownCount).finish()
    }
}
impl ::core::default::Default for STORAGE_DIAGNOSTIC_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_DIAGNOSTIC_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.ProviderId == other.ProviderId && self.BufferSize == other.BufferSize && self.Reserved == other.Reserved && self.DiagnosticDataBuffer == other.DiagnosticDataBuffer
    }
}
impl ::core::cmp::Eq for STORAGE_DIAGNOSTIC_DATA {}
impl ::core::fmt::Debug for STORAGE_DIAGNOSTIC_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DIAGNOSTIC_DATA").field("Version", &self.Version).field("Size", &self.Size).field("ProviderId", &self.ProviderId).field("BufferSize", &self.BufferSize).field("Reserved", &self.Reserved).field("DiagnosticDataBuffer", &self.DiagnosticDataBuffer).finish()
    }
}
impl ::core::default::Default for STORAGE_DIAGNOSTIC_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_DIAGNOSTIC_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_DIAGNOSTIC_LEVEL").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_DIAGNOSTIC_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_DIAGNOSTIC_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Flags == other.Flags && self.TargetType == other.TargetType && self.Level == other.Level
    }
}
impl ::core::cmp::Eq for STORAGE_DIAGNOSTIC_REQUEST {}
impl ::core::fmt::Debug for STORAGE_DIAGNOSTIC_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_DIAGNOSTIC_REQUEST").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("TargetType", &self.TargetType).field("Level", &self.Level).finish()
    }
}
impl ::core::default::Default for STORAGE_DIAGNOSTIC_TARGET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_DIAGNOSTIC_TARGET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_DIAGNOSTIC_TARGET_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_DISK_HEALTH_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_DISK_HEALTH_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_DISK_HEALTH_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_DISK_OPERATIONAL_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_DISK_OPERATIONAL_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_DISK_OPERATIONAL_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_EVENT_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_EVENT_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Events == other.Events
    }
}
impl ::core::cmp::Eq for STORAGE_EVENT_NOTIFICATION {}
impl ::core::fmt::Debug for STORAGE_EVENT_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_EVENT_NOTIFICATION").field("Version", &self.Version).field("Size", &self.Size).field("Events", &self.Events).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_FAILURE_PREDICTION_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_FAILURE_PREDICTION_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Set == other.Set && self.Enabled == other.Enabled && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_FAILURE_PREDICTION_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_FAILURE_PREDICTION_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_FAILURE_PREDICTION_CONFIG").field("Version", &self.Version).field("Size", &self.Size).field("Set", &self.Set).field("Enabled", &self.Enabled).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for STORAGE_FRU_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_FRU_ID_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.IdentifierSize == other.IdentifierSize && self.Identifier == other.Identifier
    }
}
impl ::core::cmp::Eq for STORAGE_FRU_ID_DESCRIPTOR {}
impl ::core::fmt::Debug for STORAGE_FRU_ID_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_FRU_ID_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("IdentifierSize", &self.IdentifierSize).field("Identifier", &self.Identifier).finish()
    }
}
impl ::core::default::Default for STORAGE_GET_BC_PROPERTIES_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_GET_BC_PROPERTIES_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.MaximumRequestsPerPeriod == other.MaximumRequestsPerPeriod && self.MinimumPeriod == other.MinimumPeriod && self.MaximumRequestSize == other.MaximumRequestSize && self.EstimatedTimePerRequest == other.EstimatedTimePerRequest && self.NumOutStandingRequests == other.NumOutStandingRequests && self.RequestSize == other.RequestSize
    }
}
impl ::core::cmp::Eq for STORAGE_GET_BC_PROPERTIES_OUTPUT {}
impl ::core::fmt::Debug for STORAGE_GET_BC_PROPERTIES_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_GET_BC_PROPERTIES_OUTPUT").field("MaximumRequestsPerPeriod", &self.MaximumRequestsPerPeriod).field("MinimumPeriod", &self.MinimumPeriod).field("MaximumRequestSize", &self.MaximumRequestSize).field("EstimatedTimePerRequest", &self.EstimatedTimePerRequest).field("NumOutStandingRequests", &self.NumOutStandingRequests).field("RequestSize", &self.RequestSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_HOTPLUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_HOTPLUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.MediaRemovable == other.MediaRemovable && self.MediaHotplug == other.MediaHotplug && self.DeviceHotplug == other.DeviceHotplug && self.WriteCacheEnableOverride == other.WriteCacheEnableOverride
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_HOTPLUG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_HOTPLUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HOTPLUG_INFO").field("Size", &self.Size).field("MediaRemovable", &self.MediaRemovable).field("MediaHotplug", &self.MediaHotplug).field("DeviceHotplug", &self.DeviceHotplug).field("WriteCacheEnableOverride", &self.WriteCacheEnableOverride).finish()
    }
}
impl ::core::default::Default for STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.EnduranceInfo == other.EnduranceInfo
    }
}
impl ::core::cmp::Eq for STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR {}
impl ::core::fmt::Debug for STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HW_ENDURANCE_DATA_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("EnduranceInfo", &self.EnduranceInfo).finish()
    }
}
impl ::core::default::Default for STORAGE_HW_ENDURANCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_HW_ENDURANCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ValidFields == other.ValidFields && self.GroupId == other.GroupId && self.Flags == other.Flags && self.LifePercentage == other.LifePercentage && self.BytesReadCount == other.BytesReadCount && self.ByteWriteCount == other.ByteWriteCount
    }
}
impl ::core::cmp::Eq for STORAGE_HW_ENDURANCE_INFO {}
impl ::core::fmt::Debug for STORAGE_HW_ENDURANCE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HW_ENDURANCE_INFO").field("ValidFields", &self.ValidFields).field("GroupId", &self.GroupId).field("Flags", &self.Flags).field("LifePercentage", &self.LifePercentage).field("BytesReadCount", &self.BytesReadCount).field("ByteWriteCount", &self.ByteWriteCount).finish()
    }
}
impl ::core::default::Default for STORAGE_HW_ENDURANCE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_HW_ENDURANCE_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for STORAGE_HW_ENDURANCE_INFO_0 {}
impl ::core::fmt::Debug for STORAGE_HW_ENDURANCE_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HW_ENDURANCE_INFO_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for STORAGE_HW_FIRMWARE_ACTIVATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_HW_FIRMWARE_ACTIVATE {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Flags == other.Flags && self.Slot == other.Slot && self.Reserved0 == other.Reserved0
    }
}
impl ::core::cmp::Eq for STORAGE_HW_FIRMWARE_ACTIVATE {}
impl ::core::fmt::Debug for STORAGE_HW_FIRMWARE_ACTIVATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HW_FIRMWARE_ACTIVATE").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("Slot", &self.Slot).field("Reserved0", &self.Reserved0).finish()
    }
}
impl ::core::default::Default for STORAGE_HW_FIRMWARE_DOWNLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_HW_FIRMWARE_DOWNLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Flags == other.Flags && self.Slot == other.Slot && self.Reserved == other.Reserved && self.Offset == other.Offset && self.BufferSize == other.BufferSize && self.ImageBuffer == other.ImageBuffer
    }
}
impl ::core::cmp::Eq for STORAGE_HW_FIRMWARE_DOWNLOAD {}
impl ::core::fmt::Debug for STORAGE_HW_FIRMWARE_DOWNLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HW_FIRMWARE_DOWNLOAD").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("Slot", &self.Slot).field("Reserved", &self.Reserved).field("Offset", &self.Offset).field("BufferSize", &self.BufferSize).field("ImageBuffer", &self.ImageBuffer).finish()
    }
}
impl ::core::default::Default for STORAGE_HW_FIRMWARE_DOWNLOAD_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_HW_FIRMWARE_DOWNLOAD_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Flags == other.Flags && self.Slot == other.Slot && self.Reserved == other.Reserved && self.Offset == other.Offset && self.BufferSize == other.BufferSize && self.ImageSize == other.ImageSize && self.Reserved2 == other.Reserved2 && self.ImageBuffer == other.ImageBuffer
    }
}
impl ::core::cmp::Eq for STORAGE_HW_FIRMWARE_DOWNLOAD_V2 {}
impl ::core::fmt::Debug for STORAGE_HW_FIRMWARE_DOWNLOAD_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HW_FIRMWARE_DOWNLOAD_V2").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("Slot", &self.Slot).field("Reserved", &self.Reserved).field("Offset", &self.Offset).field("BufferSize", &self.BufferSize).field("ImageSize", &self.ImageSize).field("Reserved2", &self.Reserved2).field("ImageBuffer", &self.ImageBuffer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_HW_FIRMWARE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_HW_FIRMWARE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self._bitfield == other._bitfield && self.SlotCount == other.SlotCount && self.ActiveSlot == other.ActiveSlot && self.PendingActivateSlot == other.PendingActivateSlot && self.FirmwareShared == other.FirmwareShared && self.Reserved == other.Reserved && self.ImagePayloadAlignment == other.ImagePayloadAlignment && self.ImagePayloadMaxSize == other.ImagePayloadMaxSize && self.Slot == other.Slot
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_HW_FIRMWARE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_HW_FIRMWARE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HW_FIRMWARE_INFO")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("_bitfield", &self._bitfield)
            .field("SlotCount", &self.SlotCount)
            .field("ActiveSlot", &self.ActiveSlot)
            .field("PendingActivateSlot", &self.PendingActivateSlot)
            .field("FirmwareShared", &self.FirmwareShared)
            .field("Reserved", &self.Reserved)
            .field("ImagePayloadAlignment", &self.ImagePayloadAlignment)
            .field("ImagePayloadMaxSize", &self.ImagePayloadMaxSize)
            .field("Slot", &self.Slot)
            .finish()
    }
}
impl ::core::default::Default for STORAGE_HW_FIRMWARE_INFO_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_HW_FIRMWARE_INFO_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for STORAGE_HW_FIRMWARE_INFO_QUERY {}
impl ::core::fmt::Debug for STORAGE_HW_FIRMWARE_INFO_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HW_FIRMWARE_INFO_QUERY").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for STORAGE_HW_FIRMWARE_SLOT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_HW_FIRMWARE_SLOT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.SlotNumber == other.SlotNumber && self._bitfield == other._bitfield && self.Reserved1 == other.Reserved1 && self.Revision == other.Revision
    }
}
impl ::core::cmp::Eq for STORAGE_HW_FIRMWARE_SLOT_INFO {}
impl ::core::fmt::Debug for STORAGE_HW_FIRMWARE_SLOT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_HW_FIRMWARE_SLOT_INFO").field("Version", &self.Version).field("Size", &self.Size).field("SlotNumber", &self.SlotNumber).field("_bitfield", &self._bitfield).field("Reserved1", &self.Reserved1).field("Revision", &self.Revision).finish()
    }
}
impl ::core::default::Default for STORAGE_IDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        self.CodeSet == other.CodeSet && self.Type == other.Type && self.IdentifierSize == other.IdentifierSize && self.NextOffset == other.NextOffset && self.Association == other.Association && self.Identifier == other.Identifier
    }
}
impl ::core::cmp::Eq for STORAGE_IDENTIFIER {}
impl ::core::fmt::Debug for STORAGE_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_IDENTIFIER").field("CodeSet", &self.CodeSet).field("Type", &self.Type).field("IdentifierSize", &self.IdentifierSize).field("NextOffset", &self.NextOffset).field("Association", &self.Association).field("Identifier", &self.Identifier).finish()
    }
}
impl ::core::default::Default for STORAGE_IDENTIFIER_CODE_SET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_IDENTIFIER_CODE_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_IDENTIFIER_CODE_SET").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_IDENTIFIER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_IDENTIFIER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_IDENTIFIER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_IDLE_POWER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_IDLE_POWER {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self._bitfield == other._bitfield && self.D3IdleTimeout == other.D3IdleTimeout
    }
}
impl ::core::cmp::Eq for STORAGE_IDLE_POWER {}
impl ::core::fmt::Debug for STORAGE_IDLE_POWER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_IDLE_POWER").field("Version", &self.Version).field("Size", &self.Size).field("_bitfield", &self._bitfield).field("D3IdleTimeout", &self.D3IdleTimeout).finish()
    }
}
impl ::core::default::Default for STORAGE_IDLE_POWERUP_REASON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_IDLE_POWERUP_REASON {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.PowerupReason == other.PowerupReason
    }
}
impl ::core::cmp::Eq for STORAGE_IDLE_POWERUP_REASON {}
impl ::core::fmt::Debug for STORAGE_IDLE_POWERUP_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_IDLE_POWERUP_REASON").field("Version", &self.Version).field("Size", &self.Size).field("PowerupReason", &self.PowerupReason).finish()
    }
}
impl ::core::default::Default for STORAGE_ID_NAA_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_ID_NAA_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_ID_NAA_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_LB_PROVISIONING_MAP_RESOURCES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_LB_PROVISIONING_MAP_RESOURCES {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.Version == other.Version && self._bitfield1 == other._bitfield1 && self.Reserved1 == other.Reserved1 && self._bitfield2 == other._bitfield2 && self.Reserved3 == other.Reserved3 && self.AvailableMappingResources == other.AvailableMappingResources && self.UsedMappingResources == other.UsedMappingResources
    }
}
impl ::core::cmp::Eq for STORAGE_LB_PROVISIONING_MAP_RESOURCES {}
impl ::core::fmt::Debug for STORAGE_LB_PROVISIONING_MAP_RESOURCES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_LB_PROVISIONING_MAP_RESOURCES").field("Size", &self.Size).field("Version", &self.Version).field("_bitfield1", &self._bitfield1).field("Reserved1", &self.Reserved1).field("_bitfield2", &self._bitfield2).field("Reserved3", &self.Reserved3).field("AvailableMappingResources", &self.AvailableMappingResources).field("UsedMappingResources", &self.UsedMappingResources).finish()
    }
}
impl ::core::default::Default for STORAGE_MEDIA_SERIAL_NUMBER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_MEDIA_SERIAL_NUMBER_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved == other.Reserved && self.SerialNumberLength == other.SerialNumberLength && self.SerialNumber == other.SerialNumber
    }
}
impl ::core::cmp::Eq for STORAGE_MEDIA_SERIAL_NUMBER_DATA {}
impl ::core::fmt::Debug for STORAGE_MEDIA_SERIAL_NUMBER_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_MEDIA_SERIAL_NUMBER_DATA").field("Reserved", &self.Reserved).field("SerialNumberLength", &self.SerialNumberLength).field("SerialNumber", &self.SerialNumber).finish()
    }
}
impl ::core::default::Default for STORAGE_MEDIA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_MEDIA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.MediumProductType == other.MediumProductType
    }
}
impl ::core::cmp::Eq for STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR {}
impl ::core::fmt::Debug for STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_MEDIUM_PRODUCT_TYPE_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("MediumProductType", &self.MediumProductType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_MINIPORT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STORAGE_OFFLOAD_READ_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STORAGE_OFFLOAD_TOKEN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STORAGE_OFFLOAD_WRITE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_OFFLOAD_WRITE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.OffloadWriteFlags == other.OffloadWriteFlags && self.Reserved == other.Reserved && self.LengthCopied == other.LengthCopied
    }
}
impl ::core::cmp::Eq for STORAGE_OFFLOAD_WRITE_OUTPUT {}
impl ::core::fmt::Debug for STORAGE_OFFLOAD_WRITE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_OFFLOAD_WRITE_OUTPUT").field("OffloadWriteFlags", &self.OffloadWriteFlags).field("Reserved", &self.Reserved).field("LengthCopied", &self.LengthCopied).finish()
    }
}
impl ::core::default::Default for STORAGE_OPERATIONAL_REASON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STORAGE_OPERATIONAL_STATUS_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_OPERATIONAL_STATUS_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_OPERATIONAL_STATUS_REASON").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_PHYSICAL_ADAPTER_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STORAGE_PHYSICAL_DEVICE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STORAGE_PHYSICAL_NODE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_PHYSICAL_NODE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.NodeId == other.NodeId && self.AdapterCount == other.AdapterCount && self.AdapterDataLength == other.AdapterDataLength && self.AdapterDataOffset == other.AdapterDataOffset && self.DeviceCount == other.DeviceCount && self.DeviceDataLength == other.DeviceDataLength && self.DeviceDataOffset == other.DeviceDataOffset && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for STORAGE_PHYSICAL_NODE_DATA {}
impl ::core::fmt::Debug for STORAGE_PHYSICAL_NODE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PHYSICAL_NODE_DATA").field("NodeId", &self.NodeId).field("AdapterCount", &self.AdapterCount).field("AdapterDataLength", &self.AdapterDataLength).field("AdapterDataOffset", &self.AdapterDataOffset).field("DeviceCount", &self.DeviceCount).field("DeviceDataLength", &self.DeviceDataLength).field("DeviceDataOffset", &self.DeviceDataOffset).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.NodeCount == other.NodeCount && self.Reserved == other.Reserved && self.Node == other.Node
    }
}
impl ::core::cmp::Eq for STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {}
impl ::core::fmt::Debug for STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PHYSICAL_TOPOLOGY_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("NodeCount", &self.NodeCount).field("Reserved", &self.Reserved).field("Node", &self.Node).finish()
    }
}
impl ::core::default::Default for STORAGE_PORT_CODE_SET {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_PORT_CODE_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_PORT_CODE_SET").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_POWERUP_REASON_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_POWERUP_REASON_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_POWERUP_REASON_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_PREDICT_FAILURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_PREDICT_FAILURE {
    fn eq(&self, other: &Self) -> bool {
        self.PredictFailure == other.PredictFailure && self.VendorSpecific == other.VendorSpecific
    }
}
impl ::core::cmp::Eq for STORAGE_PREDICT_FAILURE {}
impl ::core::fmt::Debug for STORAGE_PREDICT_FAILURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PREDICT_FAILURE").field("PredictFailure", &self.PredictFailure).field("VendorSpecific", &self.VendorSpecific).finish()
    }
}
impl ::core::default::Default for STORAGE_PRIORITY_HINT_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_PRIORITY_HINT_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.SupportFlags == other.SupportFlags
    }
}
impl ::core::cmp::Eq for STORAGE_PRIORITY_HINT_SUPPORT {}
impl ::core::fmt::Debug for STORAGE_PRIORITY_HINT_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PRIORITY_HINT_SUPPORT").field("SupportFlags", &self.SupportFlags).finish()
    }
}
impl ::core::default::Default for STORAGE_PROPERTY_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_PROPERTY_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_PROPERTY_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_PROPERTY_QUERY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_PROPERTY_QUERY {
    fn eq(&self, other: &Self) -> bool {
        self.PropertyId == other.PropertyId && self.QueryType == other.QueryType && self.AdditionalParameters == other.AdditionalParameters
    }
}
impl ::core::cmp::Eq for STORAGE_PROPERTY_QUERY {}
impl ::core::fmt::Debug for STORAGE_PROPERTY_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PROPERTY_QUERY").field("PropertyId", &self.PropertyId).field("QueryType", &self.QueryType).field("AdditionalParameters", &self.AdditionalParameters).finish()
    }
}
impl ::core::default::Default for STORAGE_PROPERTY_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_PROPERTY_SET {
    fn eq(&self, other: &Self) -> bool {
        self.PropertyId == other.PropertyId && self.SetType == other.SetType && self.AdditionalParameters == other.AdditionalParameters
    }
}
impl ::core::cmp::Eq for STORAGE_PROPERTY_SET {}
impl ::core::fmt::Debug for STORAGE_PROPERTY_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PROPERTY_SET").field("PropertyId", &self.PropertyId).field("SetType", &self.SetType).field("AdditionalParameters", &self.AdditionalParameters).finish()
    }
}
impl ::core::default::Default for STORAGE_PROTOCOL_ATA_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_PROTOCOL_ATA_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_PROTOCOL_ATA_DATA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_PROTOCOL_COMMAND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_PROTOCOL_COMMAND {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Length == other.Length
            && self.ProtocolType == other.ProtocolType
            && self.Flags == other.Flags
            && self.ReturnStatus == other.ReturnStatus
            && self.ErrorCode == other.ErrorCode
            && self.CommandLength == other.CommandLength
            && self.ErrorInfoLength == other.ErrorInfoLength
            && self.DataToDeviceTransferLength == other.DataToDeviceTransferLength
            && self.DataFromDeviceTransferLength == other.DataFromDeviceTransferLength
            && self.TimeOutValue == other.TimeOutValue
            && self.ErrorInfoOffset == other.ErrorInfoOffset
            && self.DataToDeviceBufferOffset == other.DataToDeviceBufferOffset
            && self.DataFromDeviceBufferOffset == other.DataFromDeviceBufferOffset
            && self.CommandSpecific == other.CommandSpecific
            && self.Reserved0 == other.Reserved0
            && self.FixedProtocolReturnData == other.FixedProtocolReturnData
            && self.Reserved1 == other.Reserved1
            && self.Command == other.Command
    }
}
impl ::core::cmp::Eq for STORAGE_PROTOCOL_COMMAND {}
impl ::core::fmt::Debug for STORAGE_PROTOCOL_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PROTOCOL_COMMAND")
            .field("Version", &self.Version)
            .field("Length", &self.Length)
            .field("ProtocolType", &self.ProtocolType)
            .field("Flags", &self.Flags)
            .field("ReturnStatus", &self.ReturnStatus)
            .field("ErrorCode", &self.ErrorCode)
            .field("CommandLength", &self.CommandLength)
            .field("ErrorInfoLength", &self.ErrorInfoLength)
            .field("DataToDeviceTransferLength", &self.DataToDeviceTransferLength)
            .field("DataFromDeviceTransferLength", &self.DataFromDeviceTransferLength)
            .field("TimeOutValue", &self.TimeOutValue)
            .field("ErrorInfoOffset", &self.ErrorInfoOffset)
            .field("DataToDeviceBufferOffset", &self.DataToDeviceBufferOffset)
            .field("DataFromDeviceBufferOffset", &self.DataFromDeviceBufferOffset)
            .field("CommandSpecific", &self.CommandSpecific)
            .field("Reserved0", &self.Reserved0)
            .field("FixedProtocolReturnData", &self.FixedProtocolReturnData)
            .field("Reserved1", &self.Reserved1)
            .field("Command", &self.Command)
            .finish()
    }
}
impl ::core::default::Default for STORAGE_PROTOCOL_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_PROTOCOL_DATA_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.ProtocolSpecificData == other.ProtocolSpecificData
    }
}
impl ::core::cmp::Eq for STORAGE_PROTOCOL_DATA_DESCRIPTOR {}
impl ::core::fmt::Debug for STORAGE_PROTOCOL_DATA_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PROTOCOL_DATA_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("ProtocolSpecificData", &self.ProtocolSpecificData).finish()
    }
}
impl ::core::default::Default for STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.ProtocolSpecificData == other.ProtocolSpecificData
    }
}
impl ::core::cmp::Eq for STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT {}
impl ::core::fmt::Debug for STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PROTOCOL_DATA_DESCRIPTOR_EXT").field("Version", &self.Version).field("Size", &self.Size).field("ProtocolSpecificData", &self.ProtocolSpecificData).finish()
    }
}
impl ::core::default::Default for STORAGE_PROTOCOL_DATA_SUBVALUE_GET_LOG_PAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STORAGE_PROTOCOL_NVME_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_PROTOCOL_NVME_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_PROTOCOL_NVME_DATA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_PROTOCOL_SPECIFIC_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_PROTOCOL_SPECIFIC_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ProtocolType == other.ProtocolType && self.DataType == other.DataType && self.ProtocolDataRequestValue == other.ProtocolDataRequestValue && self.ProtocolDataRequestSubValue == other.ProtocolDataRequestSubValue && self.ProtocolDataOffset == other.ProtocolDataOffset && self.ProtocolDataLength == other.ProtocolDataLength && self.FixedProtocolReturnData == other.FixedProtocolReturnData && self.ProtocolDataRequestSubValue2 == other.ProtocolDataRequestSubValue2 && self.ProtocolDataRequestSubValue3 == other.ProtocolDataRequestSubValue3 && self.ProtocolDataRequestSubValue4 == other.ProtocolDataRequestSubValue4
    }
}
impl ::core::cmp::Eq for STORAGE_PROTOCOL_SPECIFIC_DATA {}
impl ::core::fmt::Debug for STORAGE_PROTOCOL_SPECIFIC_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PROTOCOL_SPECIFIC_DATA")
            .field("ProtocolType", &self.ProtocolType)
            .field("DataType", &self.DataType)
            .field("ProtocolDataRequestValue", &self.ProtocolDataRequestValue)
            .field("ProtocolDataRequestSubValue", &self.ProtocolDataRequestSubValue)
            .field("ProtocolDataOffset", &self.ProtocolDataOffset)
            .field("ProtocolDataLength", &self.ProtocolDataLength)
            .field("FixedProtocolReturnData", &self.FixedProtocolReturnData)
            .field("ProtocolDataRequestSubValue2", &self.ProtocolDataRequestSubValue2)
            .field("ProtocolDataRequestSubValue3", &self.ProtocolDataRequestSubValue3)
            .field("ProtocolDataRequestSubValue4", &self.ProtocolDataRequestSubValue4)
            .finish()
    }
}
impl ::core::default::Default for STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {
    fn eq(&self, other: &Self) -> bool {
        self.ProtocolType == other.ProtocolType && self.DataType == other.DataType && self.ProtocolDataValue == other.ProtocolDataValue && self.ProtocolDataSubValue == other.ProtocolDataSubValue && self.ProtocolDataOffset == other.ProtocolDataOffset && self.ProtocolDataLength == other.ProtocolDataLength && self.FixedProtocolReturnData == other.FixedProtocolReturnData && self.ProtocolDataSubValue2 == other.ProtocolDataSubValue2 && self.ProtocolDataSubValue3 == other.ProtocolDataSubValue3 && self.ProtocolDataSubValue4 == other.ProtocolDataSubValue4 && self.ProtocolDataSubValue5 == other.ProtocolDataSubValue5 && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {}
impl ::core::fmt::Debug for STORAGE_PROTOCOL_SPECIFIC_DATA_EXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_PROTOCOL_SPECIFIC_DATA_EXT")
            .field("ProtocolType", &self.ProtocolType)
            .field("DataType", &self.DataType)
            .field("ProtocolDataValue", &self.ProtocolDataValue)
            .field("ProtocolDataSubValue", &self.ProtocolDataSubValue)
            .field("ProtocolDataOffset", &self.ProtocolDataOffset)
            .field("ProtocolDataLength", &self.ProtocolDataLength)
            .field("FixedProtocolReturnData", &self.FixedProtocolReturnData)
            .field("ProtocolDataSubValue2", &self.ProtocolDataSubValue2)
            .field("ProtocolDataSubValue3", &self.ProtocolDataSubValue3)
            .field("ProtocolDataSubValue4", &self.ProtocolDataSubValue4)
            .field("ProtocolDataSubValue5", &self.ProtocolDataSubValue5)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::core::default::Default for STORAGE_PROTOCOL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_PROTOCOL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_PROTOCOL_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_PROTOCOL_UFS_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_PROTOCOL_UFS_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_PROTOCOL_UFS_DATA_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::default::Default for STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::cmp::PartialEq for STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.EntryLength == other.EntryLength && self.DependencyTypeFlags == other.DependencyTypeFlags && self.ProviderSpecificFlags == other.ProviderSpecificFlags && self.VirtualStorageType == other.VirtualStorageType
    }
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::cmp::Eq for STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY {}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::fmt::Debug for STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_QUERY_DEPENDENT_VOLUME_LEV1_ENTRY").field("EntryLength", &self.EntryLength).field("DependencyTypeFlags", &self.DependencyTypeFlags).field("ProviderSpecificFlags", &self.ProviderSpecificFlags).field("VirtualStorageType", &self.VirtualStorageType).finish()
    }
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::default::Default for STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::cmp::PartialEq for STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.EntryLength == other.EntryLength && self.DependencyTypeFlags == other.DependencyTypeFlags && self.ProviderSpecificFlags == other.ProviderSpecificFlags && self.VirtualStorageType == other.VirtualStorageType && self.AncestorLevel == other.AncestorLevel && self.HostVolumeNameOffset == other.HostVolumeNameOffset && self.HostVolumeNameSize == other.HostVolumeNameSize && self.DependentVolumeNameOffset == other.DependentVolumeNameOffset && self.DependentVolumeNameSize == other.DependentVolumeNameSize && self.RelativePathOffset == other.RelativePathOffset && self.RelativePathSize == other.RelativePathSize && self.DependentDeviceNameOffset == other.DependentDeviceNameOffset && self.DependentDeviceNameSize == other.DependentDeviceNameSize
    }
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::cmp::Eq for STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY {}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::fmt::Debug for STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_QUERY_DEPENDENT_VOLUME_LEV2_ENTRY")
            .field("EntryLength", &self.EntryLength)
            .field("DependencyTypeFlags", &self.DependencyTypeFlags)
            .field("ProviderSpecificFlags", &self.ProviderSpecificFlags)
            .field("VirtualStorageType", &self.VirtualStorageType)
            .field("AncestorLevel", &self.AncestorLevel)
            .field("HostVolumeNameOffset", &self.HostVolumeNameOffset)
            .field("HostVolumeNameSize", &self.HostVolumeNameSize)
            .field("DependentVolumeNameOffset", &self.DependentVolumeNameOffset)
            .field("DependentVolumeNameSize", &self.DependentVolumeNameSize)
            .field("RelativePathOffset", &self.RelativePathOffset)
            .field("RelativePathSize", &self.RelativePathSize)
            .field("DependentDeviceNameOffset", &self.DependentDeviceNameOffset)
            .field("DependentDeviceNameSize", &self.DependentDeviceNameSize)
            .finish()
    }
}
impl ::core::default::Default for STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.RequestLevel == other.RequestLevel && self.RequestFlags == other.RequestFlags
    }
}
impl ::core::cmp::Eq for STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST {}
impl ::core::fmt::Debug for STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_QUERY_DEPENDENT_VOLUME_REQUEST").field("RequestLevel", &self.RequestLevel).field("RequestFlags", &self.RequestFlags).finish()
    }
}
#[cfg(feature = "Win32_Storage_Vhd")]
impl ::core::default::Default for STORAGE_QUERY_DEPENDENT_VOLUME_RESPONSE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STORAGE_QUERY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_QUERY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_QUERY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_READ_CAPACITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_READ_CAPACITY {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.BlockLength == other.BlockLength && self.NumberOfBlocks == other.NumberOfBlocks && self.DiskLength == other.DiskLength
    }
}
impl ::core::cmp::Eq for STORAGE_READ_CAPACITY {}
impl ::core::fmt::Debug for STORAGE_READ_CAPACITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_READ_CAPACITY").field("Version", &self.Version).field("Size", &self.Size).field("BlockLength", &self.BlockLength).field("NumberOfBlocks", &self.NumberOfBlocks).field("DiskLength", &self.DiskLength).finish()
    }
}
impl ::core::default::Default for STORAGE_REINITIALIZE_MEDIA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_REINITIALIZE_MEDIA {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.TimeoutInSeconds == other.TimeoutInSeconds && self.SanitizeOption == other.SanitizeOption
    }
}
impl ::core::cmp::Eq for STORAGE_REINITIALIZE_MEDIA {}
impl ::core::fmt::Debug for STORAGE_REINITIALIZE_MEDIA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_REINITIALIZE_MEDIA").field("Version", &self.Version).field("Size", &self.Size).field("TimeoutInSeconds", &self.TimeoutInSeconds).field("SanitizeOption", &self.SanitizeOption).finish()
    }
}
impl ::core::default::Default for STORAGE_REINITIALIZE_MEDIA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_REINITIALIZE_MEDIA_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for STORAGE_REINITIALIZE_MEDIA_0 {}
impl ::core::fmt::Debug for STORAGE_REINITIALIZE_MEDIA_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_REINITIALIZE_MEDIA_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::core::default::Default for STORAGE_RESERVE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_RESERVE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_RESERVE_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_RPMB_COMMAND_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_RPMB_COMMAND_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_RPMB_COMMAND_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_RPMB_DATA_FRAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_RPMB_DATA_FRAME {
    fn eq(&self, other: &Self) -> bool {
        self.Stuff == other.Stuff && self.KeyOrMAC == other.KeyOrMAC && self.Data == other.Data && self.Nonce == other.Nonce && self.WriteCounter == other.WriteCounter && self.Address == other.Address && self.BlockCount == other.BlockCount && self.OperationResult == other.OperationResult && self.RequestOrResponseType == other.RequestOrResponseType
    }
}
impl ::core::cmp::Eq for STORAGE_RPMB_DATA_FRAME {}
impl ::core::fmt::Debug for STORAGE_RPMB_DATA_FRAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_RPMB_DATA_FRAME").field("Stuff", &self.Stuff).field("KeyOrMAC", &self.KeyOrMAC).field("Data", &self.Data).field("Nonce", &self.Nonce).field("WriteCounter", &self.WriteCounter).field("Address", &self.Address).field("BlockCount", &self.BlockCount).field("OperationResult", &self.OperationResult).field("RequestOrResponseType", &self.RequestOrResponseType).finish()
    }
}
impl ::core::default::Default for STORAGE_RPMB_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_RPMB_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.SizeInBytes == other.SizeInBytes && self.MaxReliableWriteSizeInBytes == other.MaxReliableWriteSizeInBytes && self.FrameFormat == other.FrameFormat
    }
}
impl ::core::cmp::Eq for STORAGE_RPMB_DESCRIPTOR {}
impl ::core::fmt::Debug for STORAGE_RPMB_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_RPMB_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("SizeInBytes", &self.SizeInBytes).field("MaxReliableWriteSizeInBytes", &self.MaxReliableWriteSizeInBytes).field("FrameFormat", &self.FrameFormat).finish()
    }
}
impl ::core::default::Default for STORAGE_RPMB_FRAME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_RPMB_FRAME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_RPMB_FRAME_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_SANITIZE_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_SANITIZE_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_SANITIZE_METHOD").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_SET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_SET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_SET_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_SPEC_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_TEMPERATURE_DATA_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_TEMPERATURE_DATA_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.CriticalTemperature == other.CriticalTemperature && self.WarningTemperature == other.WarningTemperature && self.InfoCount == other.InfoCount && self.Reserved0 == other.Reserved0 && self.Reserved1 == other.Reserved1 && self.TemperatureInfo == other.TemperatureInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_TEMPERATURE_DATA_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_TEMPERATURE_DATA_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_TEMPERATURE_DATA_DESCRIPTOR").field("Version", &self.Version).field("Size", &self.Size).field("CriticalTemperature", &self.CriticalTemperature).field("WarningTemperature", &self.WarningTemperature).field("InfoCount", &self.InfoCount).field("Reserved0", &self.Reserved0).field("Reserved1", &self.Reserved1).field("TemperatureInfo", &self.TemperatureInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_TEMPERATURE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_TEMPERATURE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Index == other.Index && self.Temperature == other.Temperature && self.OverThreshold == other.OverThreshold && self.UnderThreshold == other.UnderThreshold && self.OverThresholdChangable == other.OverThresholdChangable && self.UnderThresholdChangable == other.UnderThresholdChangable && self.EventGenerated == other.EventGenerated && self.Reserved0 == other.Reserved0 && self.Reserved1 == other.Reserved1
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_TEMPERATURE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_TEMPERATURE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_TEMPERATURE_INFO").field("Index", &self.Index).field("Temperature", &self.Temperature).field("OverThreshold", &self.OverThreshold).field("UnderThreshold", &self.UnderThreshold).field("OverThresholdChangable", &self.OverThresholdChangable).field("UnderThresholdChangable", &self.UnderThresholdChangable).field("EventGenerated", &self.EventGenerated).field("Reserved0", &self.Reserved0).field("Reserved1", &self.Reserved1).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_TEMPERATURE_THRESHOLD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_TEMPERATURE_THRESHOLD {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.Flags == other.Flags && self.Index == other.Index && self.Threshold == other.Threshold && self.OverThreshold == other.OverThreshold && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_TEMPERATURE_THRESHOLD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_TEMPERATURE_THRESHOLD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_TEMPERATURE_THRESHOLD").field("Version", &self.Version).field("Size", &self.Size).field("Flags", &self.Flags).field("Index", &self.Index).field("Threshold", &self.Threshold).field("OverThreshold", &self.OverThreshold).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for STORAGE_TIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_TIER {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id && self.Name == other.Name && self.Description == other.Description && self.Flags == other.Flags && self.ProvisionedCapacity == other.ProvisionedCapacity && self.MediaType == other.MediaType && self.Class == other.Class
    }
}
impl ::core::cmp::Eq for STORAGE_TIER {}
impl ::core::fmt::Debug for STORAGE_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_TIER").field("Id", &self.Id).field("Name", &self.Name).field("Description", &self.Description).field("Flags", &self.Flags).field("ProvisionedCapacity", &self.ProvisionedCapacity).field("MediaType", &self.MediaType).field("Class", &self.Class).finish()
    }
}
impl ::core::default::Default for STORAGE_TIER_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_TIER_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_TIER_CLASS").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_TIER_MEDIA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_TIER_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_TIER_MEDIA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_TIER_REGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_TIER_REGION {
    fn eq(&self, other: &Self) -> bool {
        self.TierId == other.TierId && self.Offset == other.Offset && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for STORAGE_TIER_REGION {}
impl ::core::fmt::Debug for STORAGE_TIER_REGION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_TIER_REGION").field("TierId", &self.TierId).field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_WRITE_CACHE_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_WRITE_CACHE_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Size == other.Size && self.WriteCacheType == other.WriteCacheType && self.WriteCacheEnabled == other.WriteCacheEnabled && self.WriteCacheChangeable == other.WriteCacheChangeable && self.WriteThroughSupported == other.WriteThroughSupported && self.FlushCacheSupported == other.FlushCacheSupported && self.UserDefinedPowerProtection == other.UserDefinedPowerProtection && self.NVCacheEnabled == other.NVCacheEnabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_WRITE_CACHE_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_WRITE_CACHE_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_WRITE_CACHE_PROPERTY")
            .field("Version", &self.Version)
            .field("Size", &self.Size)
            .field("WriteCacheType", &self.WriteCacheType)
            .field("WriteCacheEnabled", &self.WriteCacheEnabled)
            .field("WriteCacheChangeable", &self.WriteCacheChangeable)
            .field("WriteThroughSupported", &self.WriteThroughSupported)
            .field("FlushCacheSupported", &self.FlushCacheSupported)
            .field("UserDefinedPowerProtection", &self.UserDefinedPowerProtection)
            .field("NVCacheEnabled", &self.NVCacheEnabled)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_ZONED_DEVICE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STORAGE_ZONED_DEVICE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_ZONED_DEVICE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_ZONED_DEVICE_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_ZONES_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_ZONES_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_ZONES_ATTRIBUTES").field(&self.0).finish()
    }
}
impl ::core::default::Default for STORAGE_ZONE_CONDITION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_ZONE_CONDITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_ZONE_CONDITION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STORAGE_ZONE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STORAGE_ZONE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.ZoneType == other.ZoneType && self.ZoneCondition == other.ZoneCondition && self.ResetWritePointerRecommend == other.ResetWritePointerRecommend && self.Reserved0 == other.Reserved0 && self.ZoneSize == other.ZoneSize && self.WritePointerOffset == other.WritePointerOffset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STORAGE_ZONE_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STORAGE_ZONE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ZONE_DESCRIPTOR").field("Size", &self.Size).field("ZoneType", &self.ZoneType).field("ZoneCondition", &self.ZoneCondition).field("ResetWritePointerRecommend", &self.ResetWritePointerRecommend).field("Reserved0", &self.Reserved0).field("ZoneSize", &self.ZoneSize).field("WritePointerOffset", &self.WritePointerOffset).finish()
    }
}
impl ::core::default::Default for STORAGE_ZONE_GROUP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STORAGE_ZONE_GROUP {
    fn eq(&self, other: &Self) -> bool {
        self.ZoneCount == other.ZoneCount && self.ZoneType == other.ZoneType && self.ZoneSize == other.ZoneSize
    }
}
impl ::core::cmp::Eq for STORAGE_ZONE_GROUP {}
impl ::core::fmt::Debug for STORAGE_ZONE_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STORAGE_ZONE_GROUP").field("ZoneCount", &self.ZoneCount).field("ZoneType", &self.ZoneType).field("ZoneSize", &self.ZoneSize).finish()
    }
}
impl ::core::default::Default for STORAGE_ZONE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STORAGE_ZONE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STORAGE_ZONE_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for STREAMS_ASSOCIATE_ID_INPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STREAMS_ASSOCIATE_ID_INPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.StreamId == other.StreamId
    }
}
impl ::core::cmp::Eq for STREAMS_ASSOCIATE_ID_INPUT_BUFFER {}
impl ::core::fmt::Debug for STREAMS_ASSOCIATE_ID_INPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STREAMS_ASSOCIATE_ID_INPUT_BUFFER").field("Flags", &self.Flags).field("StreamId", &self.StreamId).finish()
    }
}
impl ::core::default::Default for STREAMS_QUERY_ID_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STREAMS_QUERY_ID_OUTPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.StreamId == other.StreamId
    }
}
impl ::core::cmp::Eq for STREAMS_QUERY_ID_OUTPUT_BUFFER {}
impl ::core::fmt::Debug for STREAMS_QUERY_ID_OUTPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STREAMS_QUERY_ID_OUTPUT_BUFFER").field("StreamId", &self.StreamId).finish()
    }
}
impl ::core::default::Default for STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.OptimalWriteSize == other.OptimalWriteSize && self.StreamGranularitySize == other.StreamGranularitySize && self.StreamIdMin == other.StreamIdMin && self.StreamIdMax == other.StreamIdMax
    }
}
impl ::core::cmp::Eq for STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER {}
impl ::core::fmt::Debug for STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STREAMS_QUERY_PARAMETERS_OUTPUT_BUFFER").field("OptimalWriteSize", &self.OptimalWriteSize).field("StreamGranularitySize", &self.StreamGranularitySize).field("StreamIdMin", &self.StreamIdMin).field("StreamIdMax", &self.StreamIdMax).finish()
    }
}
impl ::core::default::Default for STREAM_EXTENT_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STREAM_INFORMATION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for STREAM_LAYOUT_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for STREAM_LAYOUT_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.NextStreamOffset == other.NextStreamOffset && self.Flags == other.Flags && self.ExtentInformationOffset == other.ExtentInformationOffset && self.AllocationSize == other.AllocationSize && self.EndOfFile == other.EndOfFile && self.StreamInformationOffset == other.StreamInformationOffset && self.AttributeTypeCode == other.AttributeTypeCode && self.AttributeFlags == other.AttributeFlags && self.StreamIdentifierLength == other.StreamIdentifierLength && self.StreamIdentifier == other.StreamIdentifier
    }
}
impl ::core::cmp::Eq for STREAM_LAYOUT_ENTRY {}
impl ::core::fmt::Debug for STREAM_LAYOUT_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STREAM_LAYOUT_ENTRY")
            .field("Version", &self.Version)
            .field("NextStreamOffset", &self.NextStreamOffset)
            .field("Flags", &self.Flags)
            .field("ExtentInformationOffset", &self.ExtentInformationOffset)
            .field("AllocationSize", &self.AllocationSize)
            .field("EndOfFile", &self.EndOfFile)
            .field("StreamInformationOffset", &self.StreamInformationOffset)
            .field("AttributeTypeCode", &self.AttributeTypeCode)
            .field("AttributeFlags", &self.AttributeFlags)
            .field("StreamIdentifierLength", &self.StreamIdentifierLength)
            .field("StreamIdentifier", &self.StreamIdentifier)
            .finish()
    }
}
impl ::core::default::Default for TAPE_GET_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TAPE_GET_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.Operation == other.Operation
    }
}
impl ::core::cmp::Eq for TAPE_GET_STATISTICS {}
impl ::core::fmt::Debug for TAPE_GET_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_GET_STATISTICS").field("Operation", &self.Operation).finish()
    }
}
impl ::core::default::Default for TAPE_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TAPE_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.RecoveredWrites == other.RecoveredWrites && self.UnrecoveredWrites == other.UnrecoveredWrites && self.RecoveredReads == other.RecoveredReads && self.UnrecoveredReads == other.UnrecoveredReads && self.CompressionRatioReads == other.CompressionRatioReads && self.CompressionRatioWrites == other.CompressionRatioWrites
    }
}
impl ::core::cmp::Eq for TAPE_STATISTICS {}
impl ::core::fmt::Debug for TAPE_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TAPE_STATISTICS").field("Version", &self.Version).field("Flags", &self.Flags).field("RecoveredWrites", &self.RecoveredWrites).field("UnrecoveredWrites", &self.UnrecoveredWrites).field("RecoveredReads", &self.RecoveredReads).field("UnrecoveredReads", &self.UnrecoveredReads).field("CompressionRatioReads", &self.CompressionRatioReads).field("CompressionRatioWrites", &self.CompressionRatioWrites).finish()
    }
}
impl ::core::default::Default for TXFS_CREATE_MINIVERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TXFS_CREATE_MINIVERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.StructureVersion == other.StructureVersion && self.StructureLength == other.StructureLength && self.BaseVersion == other.BaseVersion && self.MiniVersion == other.MiniVersion
    }
}
impl ::core::cmp::Eq for TXFS_CREATE_MINIVERSION_INFO {}
impl ::core::fmt::Debug for TXFS_CREATE_MINIVERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_CREATE_MINIVERSION_INFO").field("StructureVersion", &self.StructureVersion).field("StructureLength", &self.StructureLength).field("BaseVersion", &self.BaseVersion).field("MiniVersion", &self.MiniVersion).finish()
    }
}
impl ::core::default::Default for TXFS_GET_METADATA_INFO_OUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TXFS_GET_METADATA_INFO_OUT {
    fn eq(&self, other: &Self) -> bool {
        self.TxfFileId == other.TxfFileId && self.LockingTransaction == other.LockingTransaction && self.LastLsn == other.LastLsn && self.TransactionState == other.TransactionState
    }
}
impl ::core::cmp::Eq for TXFS_GET_METADATA_INFO_OUT {}
impl ::core::fmt::Debug for TXFS_GET_METADATA_INFO_OUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_GET_METADATA_INFO_OUT").field("TxfFileId", &self.TxfFileId).field("LockingTransaction", &self.LockingTransaction).field("LastLsn", &self.LastLsn).field("TransactionState", &self.TransactionState).finish()
    }
}
impl ::core::default::Default for TXFS_GET_METADATA_INFO_OUT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TXFS_GET_METADATA_INFO_OUT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.LowPart == other.LowPart && self.HighPart == other.HighPart
    }
}
impl ::core::cmp::Eq for TXFS_GET_METADATA_INFO_OUT_0 {}
impl ::core::fmt::Debug for TXFS_GET_METADATA_INFO_OUT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_GET_METADATA_INFO_OUT_0").field("LowPart", &self.LowPart).field("HighPart", &self.HighPart).finish()
    }
}
impl ::core::default::Default for TXFS_GET_TRANSACTED_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TXFS_GET_TRANSACTED_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.ThisBaseVersion == other.ThisBaseVersion && self.LatestVersion == other.LatestVersion && self.ThisMiniVersion == other.ThisMiniVersion && self.FirstMiniVersion == other.FirstMiniVersion && self.LatestMiniVersion == other.LatestMiniVersion
    }
}
impl ::core::cmp::Eq for TXFS_GET_TRANSACTED_VERSION {}
impl ::core::fmt::Debug for TXFS_GET_TRANSACTED_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_GET_TRANSACTED_VERSION").field("ThisBaseVersion", &self.ThisBaseVersion).field("LatestVersion", &self.LatestVersion).field("ThisMiniVersion", &self.ThisMiniVersion).field("FirstMiniVersion", &self.FirstMiniVersion).field("LatestMiniVersion", &self.LatestMiniVersion).finish()
    }
}
impl ::core::default::Default for TXFS_LIST_TRANSACTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TXFS_LIST_TRANSACTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfTransactions == other.NumberOfTransactions && self.BufferSizeRequired == other.BufferSizeRequired
    }
}
impl ::core::cmp::Eq for TXFS_LIST_TRANSACTIONS {}
impl ::core::fmt::Debug for TXFS_LIST_TRANSACTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_LIST_TRANSACTIONS").field("NumberOfTransactions", &self.NumberOfTransactions).field("BufferSizeRequired", &self.BufferSizeRequired).finish()
    }
}
impl ::core::default::Default for TXFS_LIST_TRANSACTIONS_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TXFS_LIST_TRANSACTIONS_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.TransactionId == other.TransactionId && self.TransactionState == other.TransactionState && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.Reserved3 == other.Reserved3
    }
}
impl ::core::cmp::Eq for TXFS_LIST_TRANSACTIONS_ENTRY {}
impl ::core::fmt::Debug for TXFS_LIST_TRANSACTIONS_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_LIST_TRANSACTIONS_ENTRY").field("TransactionId", &self.TransactionId).field("TransactionState", &self.TransactionState).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).finish()
    }
}
impl ::core::default::Default for TXFS_LIST_TRANSACTION_LOCKED_FILES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TXFS_LIST_TRANSACTION_LOCKED_FILES {
    fn eq(&self, other: &Self) -> bool {
        self.KtmTransaction == other.KtmTransaction && self.NumberOfFiles == other.NumberOfFiles && self.BufferSizeRequired == other.BufferSizeRequired && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for TXFS_LIST_TRANSACTION_LOCKED_FILES {}
impl ::core::fmt::Debug for TXFS_LIST_TRANSACTION_LOCKED_FILES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_LIST_TRANSACTION_LOCKED_FILES").field("KtmTransaction", &self.KtmTransaction).field("NumberOfFiles", &self.NumberOfFiles).field("BufferSizeRequired", &self.BufferSizeRequired).field("Offset", &self.Offset).finish()
    }
}
impl ::core::default::Default for TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.NameFlags == other.NameFlags && self.FileId == other.FileId && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.Reserved3 == other.Reserved3 && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {}
impl ::core::fmt::Debug for TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_LIST_TRANSACTION_LOCKED_FILES_ENTRY").field("Offset", &self.Offset).field("NameFlags", &self.NameFlags).field("FileId", &self.FileId).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).field("FileName", &self.FileName).finish()
    }
}
impl ::core::default::Default for TXFS_MODIFY_RM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TXFS_MODIFY_RM {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.LogContainerCountMax == other.LogContainerCountMax && self.LogContainerCountMin == other.LogContainerCountMin && self.LogContainerCount == other.LogContainerCount && self.LogGrowthIncrement == other.LogGrowthIncrement && self.LogAutoShrinkPercentage == other.LogAutoShrinkPercentage && self.Reserved == other.Reserved && self.LoggingMode == other.LoggingMode
    }
}
impl ::core::cmp::Eq for TXFS_MODIFY_RM {}
impl ::core::fmt::Debug for TXFS_MODIFY_RM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_MODIFY_RM").field("Flags", &self.Flags).field("LogContainerCountMax", &self.LogContainerCountMax).field("LogContainerCountMin", &self.LogContainerCountMin).field("LogContainerCount", &self.LogContainerCount).field("LogGrowthIncrement", &self.LogGrowthIncrement).field("LogAutoShrinkPercentage", &self.LogAutoShrinkPercentage).field("Reserved", &self.Reserved).field("LoggingMode", &self.LoggingMode).finish()
    }
}
impl ::core::default::Default for TXFS_QUERY_RM_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TXFS_QUERY_RM_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.BytesRequired == other.BytesRequired
            && self.TailLsn == other.TailLsn
            && self.CurrentLsn == other.CurrentLsn
            && self.ArchiveTailLsn == other.ArchiveTailLsn
            && self.LogContainerSize == other.LogContainerSize
            && self.HighestVirtualClock == other.HighestVirtualClock
            && self.LogContainerCount == other.LogContainerCount
            && self.LogContainerCountMax == other.LogContainerCountMax
            && self.LogContainerCountMin == other.LogContainerCountMin
            && self.LogGrowthIncrement == other.LogGrowthIncrement
            && self.LogAutoShrinkPercentage == other.LogAutoShrinkPercentage
            && self.Flags == other.Flags
            && self.LoggingMode == other.LoggingMode
            && self.Reserved == other.Reserved
            && self.RmState == other.RmState
            && self.LogCapacity == other.LogCapacity
            && self.LogFree == other.LogFree
            && self.TopsSize == other.TopsSize
            && self.TopsUsed == other.TopsUsed
            && self.TransactionCount == other.TransactionCount
            && self.OnePCCount == other.OnePCCount
            && self.TwoPCCount == other.TwoPCCount
            && self.NumberLogFileFull == other.NumberLogFileFull
            && self.OldestTransactionAge == other.OldestTransactionAge
            && self.RMName == other.RMName
            && self.TmLogPathOffset == other.TmLogPathOffset
    }
}
impl ::core::cmp::Eq for TXFS_QUERY_RM_INFORMATION {}
impl ::core::fmt::Debug for TXFS_QUERY_RM_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_QUERY_RM_INFORMATION")
            .field("BytesRequired", &self.BytesRequired)
            .field("TailLsn", &self.TailLsn)
            .field("CurrentLsn", &self.CurrentLsn)
            .field("ArchiveTailLsn", &self.ArchiveTailLsn)
            .field("LogContainerSize", &self.LogContainerSize)
            .field("HighestVirtualClock", &self.HighestVirtualClock)
            .field("LogContainerCount", &self.LogContainerCount)
            .field("LogContainerCountMax", &self.LogContainerCountMax)
            .field("LogContainerCountMin", &self.LogContainerCountMin)
            .field("LogGrowthIncrement", &self.LogGrowthIncrement)
            .field("LogAutoShrinkPercentage", &self.LogAutoShrinkPercentage)
            .field("Flags", &self.Flags)
            .field("LoggingMode", &self.LoggingMode)
            .field("Reserved", &self.Reserved)
            .field("RmState", &self.RmState)
            .field("LogCapacity", &self.LogCapacity)
            .field("LogFree", &self.LogFree)
            .field("TopsSize", &self.TopsSize)
            .field("TopsUsed", &self.TopsUsed)
            .field("TransactionCount", &self.TransactionCount)
            .field("OnePCCount", &self.OnePCCount)
            .field("TwoPCCount", &self.TwoPCCount)
            .field("NumberLogFileFull", &self.NumberLogFileFull)
            .field("OldestTransactionAge", &self.OldestTransactionAge)
            .field("RMName", &self.RMName)
            .field("TmLogPathOffset", &self.TmLogPathOffset)
            .finish()
    }
}
impl ::core::default::Default for TXFS_READ_BACKUP_INFORMATION_OUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for TXFS_RMF_LAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for TXFS_RMF_LAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TXFS_RMF_LAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for TXFS_RMF_LAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TXFS_RMF_LAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TXFS_RMF_LAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TXFS_RMF_LAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TXFS_RMF_LAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for TXFS_ROLLFORWARD_REDO_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TXFS_ROLLFORWARD_REDO_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.LastVirtualClock == other.LastVirtualClock && self.LastRedoLsn == other.LastRedoLsn && self.HighestRecoveryLsn == other.HighestRecoveryLsn && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for TXFS_ROLLFORWARD_REDO_INFORMATION {}
impl ::core::fmt::Debug for TXFS_ROLLFORWARD_REDO_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_ROLLFORWARD_REDO_INFORMATION").field("LastVirtualClock", &self.LastVirtualClock).field("LastRedoLsn", &self.LastRedoLsn).field("HighestRecoveryLsn", &self.HighestRecoveryLsn).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TXFS_SAVEPOINT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TXFS_SAVEPOINT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.KtmTransaction == other.KtmTransaction && self.ActionCode == other.ActionCode && self.SavepointId == other.SavepointId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TXFS_SAVEPOINT_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TXFS_SAVEPOINT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_SAVEPOINT_INFORMATION").field("KtmTransaction", &self.KtmTransaction).field("ActionCode", &self.ActionCode).field("SavepointId", &self.SavepointId).finish()
    }
}
impl ::core::default::Default for TXFS_START_RM_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TXFS_START_RM_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.LogContainerSize == other.LogContainerSize && self.LogContainerCountMin == other.LogContainerCountMin && self.LogContainerCountMax == other.LogContainerCountMax && self.LogGrowthIncrement == other.LogGrowthIncrement && self.LogAutoShrinkPercentage == other.LogAutoShrinkPercentage && self.TmLogPathOffset == other.TmLogPathOffset && self.TmLogPathLength == other.TmLogPathLength && self.LoggingMode == other.LoggingMode && self.LogPathLength == other.LogPathLength && self.Reserved == other.Reserved && self.LogPath == other.LogPath
    }
}
impl ::core::cmp::Eq for TXFS_START_RM_INFORMATION {}
impl ::core::fmt::Debug for TXFS_START_RM_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_START_RM_INFORMATION")
            .field("Flags", &self.Flags)
            .field("LogContainerSize", &self.LogContainerSize)
            .field("LogContainerCountMin", &self.LogContainerCountMin)
            .field("LogContainerCountMax", &self.LogContainerCountMax)
            .field("LogGrowthIncrement", &self.LogGrowthIncrement)
            .field("LogAutoShrinkPercentage", &self.LogAutoShrinkPercentage)
            .field("TmLogPathOffset", &self.TmLogPathOffset)
            .field("TmLogPathLength", &self.TmLogPathLength)
            .field("LoggingMode", &self.LoggingMode)
            .field("LogPathLength", &self.LogPathLength)
            .field("Reserved", &self.Reserved)
            .field("LogPath", &self.LogPath)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for TXFS_TRANSACTION_ACTIVE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for TXFS_TRANSACTION_ACTIVE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.TransactionsActiveAtSnapshot == other.TransactionsActiveAtSnapshot
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for TXFS_TRANSACTION_ACTIVE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for TXFS_TRANSACTION_ACTIVE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_TRANSACTION_ACTIVE_INFO").field("TransactionsActiveAtSnapshot", &self.TransactionsActiveAtSnapshot).finish()
    }
}
impl ::core::default::Default for TXFS_WRITE_BACKUP_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for TXFS_WRITE_BACKUP_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for TXFS_WRITE_BACKUP_INFORMATION {}
impl ::core::fmt::Debug for TXFS_WRITE_BACKUP_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TXFS_WRITE_BACKUP_INFORMATION").field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for USN_DELETE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USN_DELETE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USN_DELETE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for USN_DELETE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for USN_DELETE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for USN_DELETE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for USN_DELETE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for USN_DELETE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for USN_JOURNAL_DATA_V0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USN_JOURNAL_DATA_V0 {
    fn eq(&self, other: &Self) -> bool {
        self.UsnJournalID == other.UsnJournalID && self.FirstUsn == other.FirstUsn && self.NextUsn == other.NextUsn && self.LowestValidUsn == other.LowestValidUsn && self.MaxUsn == other.MaxUsn && self.MaximumSize == other.MaximumSize && self.AllocationDelta == other.AllocationDelta
    }
}
impl ::core::cmp::Eq for USN_JOURNAL_DATA_V0 {}
impl ::core::fmt::Debug for USN_JOURNAL_DATA_V0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_JOURNAL_DATA_V0").field("UsnJournalID", &self.UsnJournalID).field("FirstUsn", &self.FirstUsn).field("NextUsn", &self.NextUsn).field("LowestValidUsn", &self.LowestValidUsn).field("MaxUsn", &self.MaxUsn).field("MaximumSize", &self.MaximumSize).field("AllocationDelta", &self.AllocationDelta).finish()
    }
}
impl ::core::default::Default for USN_JOURNAL_DATA_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USN_JOURNAL_DATA_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.UsnJournalID == other.UsnJournalID && self.FirstUsn == other.FirstUsn && self.NextUsn == other.NextUsn && self.LowestValidUsn == other.LowestValidUsn && self.MaxUsn == other.MaxUsn && self.MaximumSize == other.MaximumSize && self.AllocationDelta == other.AllocationDelta && self.MinSupportedMajorVersion == other.MinSupportedMajorVersion && self.MaxSupportedMajorVersion == other.MaxSupportedMajorVersion
    }
}
impl ::core::cmp::Eq for USN_JOURNAL_DATA_V1 {}
impl ::core::fmt::Debug for USN_JOURNAL_DATA_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_JOURNAL_DATA_V1").field("UsnJournalID", &self.UsnJournalID).field("FirstUsn", &self.FirstUsn).field("NextUsn", &self.NextUsn).field("LowestValidUsn", &self.LowestValidUsn).field("MaxUsn", &self.MaxUsn).field("MaximumSize", &self.MaximumSize).field("AllocationDelta", &self.AllocationDelta).field("MinSupportedMajorVersion", &self.MinSupportedMajorVersion).field("MaxSupportedMajorVersion", &self.MaxSupportedMajorVersion).finish()
    }
}
impl ::core::default::Default for USN_JOURNAL_DATA_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USN_JOURNAL_DATA_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.UsnJournalID == other.UsnJournalID && self.FirstUsn == other.FirstUsn && self.NextUsn == other.NextUsn && self.LowestValidUsn == other.LowestValidUsn && self.MaxUsn == other.MaxUsn && self.MaximumSize == other.MaximumSize && self.AllocationDelta == other.AllocationDelta && self.MinSupportedMajorVersion == other.MinSupportedMajorVersion && self.MaxSupportedMajorVersion == other.MaxSupportedMajorVersion && self.Flags == other.Flags && self.RangeTrackChunkSize == other.RangeTrackChunkSize && self.RangeTrackFileSizeThreshold == other.RangeTrackFileSizeThreshold
    }
}
impl ::core::cmp::Eq for USN_JOURNAL_DATA_V2 {}
impl ::core::fmt::Debug for USN_JOURNAL_DATA_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_JOURNAL_DATA_V2")
            .field("UsnJournalID", &self.UsnJournalID)
            .field("FirstUsn", &self.FirstUsn)
            .field("NextUsn", &self.NextUsn)
            .field("LowestValidUsn", &self.LowestValidUsn)
            .field("MaxUsn", &self.MaxUsn)
            .field("MaximumSize", &self.MaximumSize)
            .field("AllocationDelta", &self.AllocationDelta)
            .field("MinSupportedMajorVersion", &self.MinSupportedMajorVersion)
            .field("MaxSupportedMajorVersion", &self.MaxSupportedMajorVersion)
            .field("Flags", &self.Flags)
            .field("RangeTrackChunkSize", &self.RangeTrackChunkSize)
            .field("RangeTrackFileSizeThreshold", &self.RangeTrackFileSizeThreshold)
            .finish()
    }
}
impl ::core::default::Default for USN_RANGE_TRACK_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USN_RANGE_TRACK_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Usn == other.Usn
    }
}
impl ::core::cmp::Eq for USN_RANGE_TRACK_OUTPUT {}
impl ::core::fmt::Debug for USN_RANGE_TRACK_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_RANGE_TRACK_OUTPUT").field("Usn", &self.Usn).finish()
    }
}
impl ::core::default::Default for USN_RECORD_COMMON_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USN_RECORD_COMMON_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.RecordLength == other.RecordLength && self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion
    }
}
impl ::core::cmp::Eq for USN_RECORD_COMMON_HEADER {}
impl ::core::fmt::Debug for USN_RECORD_COMMON_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_RECORD_COMMON_HEADER").field("RecordLength", &self.RecordLength).field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
impl ::core::default::Default for USN_RECORD_EXTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USN_RECORD_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for USN_RECORD_EXTENT {}
impl ::core::fmt::Debug for USN_RECORD_EXTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_RECORD_EXTENT").field("Offset", &self.Offset).field("Length", &self.Length).finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for USN_RECORD_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for USN_RECORD_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USN_RECORD_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.RecordLength == other.RecordLength && self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.FileReferenceNumber == other.FileReferenceNumber && self.ParentFileReferenceNumber == other.ParentFileReferenceNumber && self.Usn == other.Usn && self.TimeStamp == other.TimeStamp && self.Reason == other.Reason && self.SourceInfo == other.SourceInfo && self.SecurityId == other.SecurityId && self.FileAttributes == other.FileAttributes && self.FileNameLength == other.FileNameLength && self.FileNameOffset == other.FileNameOffset && self.FileName == other.FileName
    }
}
impl ::core::cmp::Eq for USN_RECORD_V2 {}
impl ::core::fmt::Debug for USN_RECORD_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_RECORD_V2")
            .field("RecordLength", &self.RecordLength)
            .field("MajorVersion", &self.MajorVersion)
            .field("MinorVersion", &self.MinorVersion)
            .field("FileReferenceNumber", &self.FileReferenceNumber)
            .field("ParentFileReferenceNumber", &self.ParentFileReferenceNumber)
            .field("Usn", &self.Usn)
            .field("TimeStamp", &self.TimeStamp)
            .field("Reason", &self.Reason)
            .field("SourceInfo", &self.SourceInfo)
            .field("SecurityId", &self.SecurityId)
            .field("FileAttributes", &self.FileAttributes)
            .field("FileNameLength", &self.FileNameLength)
            .field("FileNameOffset", &self.FileNameOffset)
            .field("FileName", &self.FileName)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for USN_RECORD_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for USN_RECORD_V3 {
    fn eq(&self, other: &Self) -> bool {
        self.RecordLength == other.RecordLength && self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.FileReferenceNumber == other.FileReferenceNumber && self.ParentFileReferenceNumber == other.ParentFileReferenceNumber && self.Usn == other.Usn && self.TimeStamp == other.TimeStamp && self.Reason == other.Reason && self.SourceInfo == other.SourceInfo && self.SecurityId == other.SecurityId && self.FileAttributes == other.FileAttributes && self.FileNameLength == other.FileNameLength && self.FileNameOffset == other.FileNameOffset && self.FileName == other.FileName
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for USN_RECORD_V3 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::fmt::Debug for USN_RECORD_V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_RECORD_V3")
            .field("RecordLength", &self.RecordLength)
            .field("MajorVersion", &self.MajorVersion)
            .field("MinorVersion", &self.MinorVersion)
            .field("FileReferenceNumber", &self.FileReferenceNumber)
            .field("ParentFileReferenceNumber", &self.ParentFileReferenceNumber)
            .field("Usn", &self.Usn)
            .field("TimeStamp", &self.TimeStamp)
            .field("Reason", &self.Reason)
            .field("SourceInfo", &self.SourceInfo)
            .field("SecurityId", &self.SecurityId)
            .field("FileAttributes", &self.FileAttributes)
            .field("FileNameLength", &self.FileNameLength)
            .field("FileNameOffset", &self.FileNameOffset)
            .field("FileName", &self.FileName)
            .finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for USN_RECORD_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for USN_RECORD_V4 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.FileReferenceNumber == other.FileReferenceNumber && self.ParentFileReferenceNumber == other.ParentFileReferenceNumber && self.Usn == other.Usn && self.Reason == other.Reason && self.SourceInfo == other.SourceInfo && self.RemainingExtents == other.RemainingExtents && self.NumberOfExtents == other.NumberOfExtents && self.ExtentSize == other.ExtentSize && self.Extents == other.Extents
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for USN_RECORD_V4 {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::fmt::Debug for USN_RECORD_V4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_RECORD_V4").field("Header", &self.Header).field("FileReferenceNumber", &self.FileReferenceNumber).field("ParentFileReferenceNumber", &self.ParentFileReferenceNumber).field("Usn", &self.Usn).field("Reason", &self.Reason).field("SourceInfo", &self.SourceInfo).field("RemainingExtents", &self.RemainingExtents).field("NumberOfExtents", &self.NumberOfExtents).field("ExtentSize", &self.ExtentSize).field("Extents", &self.Extents).finish()
    }
}
impl ::core::default::Default for USN_SOURCE_INFO_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for USN_SOURCE_INFO_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USN_SOURCE_INFO_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for USN_TRACK_MODIFIED_RANGES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for USN_TRACK_MODIFIED_RANGES {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Unused == other.Unused && self.ChunkSize == other.ChunkSize && self.FileSizeThreshold == other.FileSizeThreshold
    }
}
impl ::core::cmp::Eq for USN_TRACK_MODIFIED_RANGES {}
impl ::core::fmt::Debug for USN_TRACK_MODIFIED_RANGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USN_TRACK_MODIFIED_RANGES").field("Flags", &self.Flags).field("Unused", &self.Unused).field("ChunkSize", &self.ChunkSize).field("FileSizeThreshold", &self.FileSizeThreshold).finish()
    }
}
impl ::core::default::Default for VERIFY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VERIFY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.StartingOffset == other.StartingOffset && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for VERIFY_INFORMATION {}
impl ::core::fmt::Debug for VERIFY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VERIFY_INFORMATION").field("StartingOffset", &self.StartingOffset).field("Length", &self.Length).finish()
    }
}
impl ::core::default::Default for VIRTUALIZATION_INSTANCE_INFO_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIRTUALIZATION_INSTANCE_INFO_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfWorkerThreads == other.NumberOfWorkerThreads && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for VIRTUALIZATION_INSTANCE_INFO_INPUT {}
impl ::core::fmt::Debug for VIRTUALIZATION_INSTANCE_INFO_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIRTUALIZATION_INSTANCE_INFO_INPUT").field("NumberOfWorkerThreads", &self.NumberOfWorkerThreads).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for VIRTUALIZATION_INSTANCE_INFO_INPUT_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIRTUALIZATION_INSTANCE_INFO_INPUT_EX {
    fn eq(&self, other: &Self) -> bool {
        self.HeaderSize == other.HeaderSize && self.Flags == other.Flags && self.NotificationInfoSize == other.NotificationInfoSize && self.NotificationInfoOffset == other.NotificationInfoOffset && self.ProviderMajorVersion == other.ProviderMajorVersion
    }
}
impl ::core::cmp::Eq for VIRTUALIZATION_INSTANCE_INFO_INPUT_EX {}
impl ::core::fmt::Debug for VIRTUALIZATION_INSTANCE_INFO_INPUT_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIRTUALIZATION_INSTANCE_INFO_INPUT_EX").field("HeaderSize", &self.HeaderSize).field("Flags", &self.Flags).field("NotificationInfoSize", &self.NotificationInfoSize).field("NotificationInfoOffset", &self.NotificationInfoOffset).field("ProviderMajorVersion", &self.ProviderMajorVersion).finish()
    }
}
impl ::core::default::Default for VIRTUALIZATION_INSTANCE_INFO_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIRTUALIZATION_INSTANCE_INFO_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualizationInstanceID == other.VirtualizationInstanceID
    }
}
impl ::core::cmp::Eq for VIRTUALIZATION_INSTANCE_INFO_OUTPUT {}
impl ::core::fmt::Debug for VIRTUALIZATION_INSTANCE_INFO_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIRTUALIZATION_INSTANCE_INFO_OUTPUT").field("VirtualizationInstanceID", &self.VirtualizationInstanceID).finish()
    }
}
impl ::core::default::Default for VIRTUAL_STORAGE_BEHAVIOR_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for VIRTUAL_STORAGE_BEHAVIOR_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VIRTUAL_STORAGE_BEHAVIOR_CODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size && self.BehaviorCode == other.BehaviorCode
    }
}
impl ::core::cmp::Eq for VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT {}
impl ::core::fmt::Debug for VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VIRTUAL_STORAGE_SET_BEHAVIOR_INPUT").field("Size", &self.Size).field("BehaviorCode", &self.BehaviorCode).finish()
    }
}
impl ::core::default::Default for VOLUME_BITMAP_BUFFER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VOLUME_BITMAP_BUFFER {
    fn eq(&self, other: &Self) -> bool {
        self.StartingLcn == other.StartingLcn && self.BitmapSize == other.BitmapSize && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for VOLUME_BITMAP_BUFFER {}
impl ::core::fmt::Debug for VOLUME_BITMAP_BUFFER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_BITMAP_BUFFER").field("StartingLcn", &self.StartingLcn).field("BitmapSize", &self.BitmapSize).field("Buffer", &self.Buffer).finish()
    }
}
impl ::core::default::Default for VOLUME_DISK_EXTENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VOLUME_DISK_EXTENTS {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfDiskExtents == other.NumberOfDiskExtents && self.Extents == other.Extents
    }
}
impl ::core::cmp::Eq for VOLUME_DISK_EXTENTS {}
impl ::core::fmt::Debug for VOLUME_DISK_EXTENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_DISK_EXTENTS").field("NumberOfDiskExtents", &self.NumberOfDiskExtents).field("Extents", &self.Extents).finish()
    }
}
impl ::core::default::Default for VOLUME_GET_GPT_ATTRIBUTES_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for VOLUME_GET_GPT_ATTRIBUTES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.GptAttributes == other.GptAttributes
    }
}
impl ::core::cmp::Eq for VOLUME_GET_GPT_ATTRIBUTES_INFORMATION {}
impl ::core::fmt::Debug for VOLUME_GET_GPT_ATTRIBUTES_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VOLUME_GET_GPT_ATTRIBUTES_INFORMATION").field("GptAttributes", &self.GptAttributes).finish()
    }
}
impl ::core::default::Default for WIM_PROVIDER_ADD_OVERLAY_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIM_PROVIDER_ADD_OVERLAY_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.WimType == other.WimType && self.WimIndex == other.WimIndex && self.WimFileNameOffset == other.WimFileNameOffset && self.WimFileNameLength == other.WimFileNameLength
    }
}
impl ::core::cmp::Eq for WIM_PROVIDER_ADD_OVERLAY_INPUT {}
impl ::core::fmt::Debug for WIM_PROVIDER_ADD_OVERLAY_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIM_PROVIDER_ADD_OVERLAY_INPUT").field("WimType", &self.WimType).field("WimIndex", &self.WimIndex).field("WimFileNameOffset", &self.WimFileNameOffset).field("WimFileNameLength", &self.WimFileNameLength).finish()
    }
}
impl ::core::default::Default for WIM_PROVIDER_EXTERNAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIM_PROVIDER_EXTERNAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.DataSourceId == other.DataSourceId && self.ResourceHash == other.ResourceHash
    }
}
impl ::core::cmp::Eq for WIM_PROVIDER_EXTERNAL_INFO {}
impl ::core::fmt::Debug for WIM_PROVIDER_EXTERNAL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIM_PROVIDER_EXTERNAL_INFO").field("Version", &self.Version).field("Flags", &self.Flags).field("DataSourceId", &self.DataSourceId).field("ResourceHash", &self.ResourceHash).finish()
    }
}
impl ::core::default::Default for WIM_PROVIDER_OVERLAY_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIM_PROVIDER_OVERLAY_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.DataSourceId == other.DataSourceId && self.WimGuid == other.WimGuid && self.WimFileNameOffset == other.WimFileNameOffset && self.WimType == other.WimType && self.WimIndex == other.WimIndex && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for WIM_PROVIDER_OVERLAY_ENTRY {}
impl ::core::fmt::Debug for WIM_PROVIDER_OVERLAY_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIM_PROVIDER_OVERLAY_ENTRY").field("NextEntryOffset", &self.NextEntryOffset).field("DataSourceId", &self.DataSourceId).field("WimGuid", &self.WimGuid).field("WimFileNameOffset", &self.WimFileNameOffset).field("WimType", &self.WimType).field("WimIndex", &self.WimIndex).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for WIM_PROVIDER_REMOVE_OVERLAY_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIM_PROVIDER_REMOVE_OVERLAY_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.DataSourceId == other.DataSourceId
    }
}
impl ::core::cmp::Eq for WIM_PROVIDER_REMOVE_OVERLAY_INPUT {}
impl ::core::fmt::Debug for WIM_PROVIDER_REMOVE_OVERLAY_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIM_PROVIDER_REMOVE_OVERLAY_INPUT").field("DataSourceId", &self.DataSourceId).finish()
    }
}
impl ::core::default::Default for WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.DataSourceId == other.DataSourceId
    }
}
impl ::core::cmp::Eq for WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {}
impl ::core::fmt::Debug for WIM_PROVIDER_SUSPEND_OVERLAY_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIM_PROVIDER_SUSPEND_OVERLAY_INPUT").field("DataSourceId", &self.DataSourceId).finish()
    }
}
impl ::core::default::Default for WIM_PROVIDER_UPDATE_OVERLAY_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WIM_PROVIDER_UPDATE_OVERLAY_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.DataSourceId == other.DataSourceId && self.WimFileNameOffset == other.WimFileNameOffset && self.WimFileNameLength == other.WimFileNameLength
    }
}
impl ::core::cmp::Eq for WIM_PROVIDER_UPDATE_OVERLAY_INPUT {}
impl ::core::fmt::Debug for WIM_PROVIDER_UPDATE_OVERLAY_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WIM_PROVIDER_UPDATE_OVERLAY_INPUT").field("DataSourceId", &self.DataSourceId).field("WimFileNameOffset", &self.WimFileNameOffset).field("WimFileNameLength", &self.WimFileNameLength).finish()
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::default::Default for WOF_EXTERNAL_FILE_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::PartialEq for WOF_EXTERNAL_FILE_ID {
    fn eq(&self, other: &Self) -> bool {
        self.FileId == other.FileId
    }
}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::cmp::Eq for WOF_EXTERNAL_FILE_ID {}
#[cfg(feature = "Win32_Storage_FileSystem")]
impl ::core::fmt::Debug for WOF_EXTERNAL_FILE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WOF_EXTERNAL_FILE_ID").field("FileId", &self.FileId).finish()
    }
}
impl ::core::default::Default for WOF_EXTERNAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WOF_EXTERNAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Provider == other.Provider
    }
}
impl ::core::cmp::Eq for WOF_EXTERNAL_INFO {}
impl ::core::fmt::Debug for WOF_EXTERNAL_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WOF_EXTERNAL_INFO").field("Version", &self.Version).field("Provider", &self.Provider).finish()
    }
}
impl ::core::default::Default for WOF_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WOF_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.WofVersion == other.WofVersion
    }
}
impl ::core::cmp::Eq for WOF_VERSION_INFO {}
impl ::core::fmt::Debug for WOF_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WOF_VERSION_INFO").field("WofVersion", &self.WofVersion).finish()
    }
}
impl ::core::default::Default for WRITE_CACHE_CHANGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WRITE_CACHE_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRITE_CACHE_CHANGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WRITE_CACHE_ENABLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WRITE_CACHE_ENABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRITE_CACHE_ENABLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WRITE_CACHE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WRITE_CACHE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRITE_CACHE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WRITE_THROUGH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WRITE_THROUGH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WRITE_THROUGH").field(&self.0).finish()
    }
}
impl ::core::default::Default for WRITE_USN_REASON_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WRITE_USN_REASON_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.UsnReasonToWrite == other.UsnReasonToWrite
    }
}
impl ::core::cmp::Eq for WRITE_USN_REASON_INPUT {}
impl ::core::fmt::Debug for WRITE_USN_REASON_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WRITE_USN_REASON_INPUT").field("Flags", &self.Flags).field("UsnReasonToWrite", &self.UsnReasonToWrite).finish()
    }
}
