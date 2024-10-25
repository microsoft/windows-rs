pub const E_DELTA_PRINTTICKET_FORMAT: u32 = 2147745797u32;
pub const E_PRINTCAPABILITIES_FORMAT: u32 = 2147745796u32;
pub const E_PRINTDEVICECAPABILITIES_FORMAT: u32 = 2147745798u32;
pub const E_PRINTTICKET_FORMAT: u32 = 2147745795u32;
pub const PRINTTICKET_ISTREAM_APIS: u32 = 1u32;
pub const S_PT_CONFLICT_RESOLVED: u32 = 262146u32;
pub const S_PT_NO_CONFLICT: u32 = 262145u32;
pub const kPTDocumentScope: EPrintTicketScope = 1i32;
pub const kPTJobScope: EPrintTicketScope = 2i32;
pub const kPTPageScope: EPrintTicketScope = 0i32;
pub const kPrinterDefaultDevmode: EDefaultDevmodeType = 1i32;
pub const kUserDefaultDevmode: EDefaultDevmodeType = 0i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EDefaultDevmodeType(pub i32);
impl windows_core::TypeKind for EDefaultDevmodeType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EPrintTicketScope(pub i32);
impl windows_core::TypeKind for EPrintTicketScope {
    type TypeKind = windows_core::CopyType;
}
