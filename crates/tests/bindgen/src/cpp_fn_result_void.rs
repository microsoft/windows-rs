#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[inline]
pub unsafe fn SetComputerNameA<P0>(lpcomputername: P0) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetComputerNameA(lpcomputername : windows_core::PCSTR) -> BOOL);
    SetComputerNameA(lpcomputername.param().abi()).ok()
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct BOOL(pub i32);
impl windows_core::TypeKind for BOOL {
    type TypeKind = windows_core::CopyType;
}
