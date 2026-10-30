/// Architecture bits from `SupportedArchitectureAttribute`: x86 = 1, x64 = 2, arm64 = 4. Metadata
/// uses `0` for items available on every architecture.
pub const X86: i32 = 1;
pub const ALL_ARCHES: i32 = 7;

/// Returns the architectures covered by `bits`, expanding the metadata `0` to all of them.
pub fn arch_mask(bits: i32) -> i32 {
    if bits == 0 { ALL_ARCHES } else { bits }
}

/// Iterates the individual architecture bits covered by `bits`.
pub fn each_arch(bits: i32) -> impl Iterator<Item = i32> {
    let mask = arch_mask(bits);
    [1, 2, 4].into_iter().filter(move |arch| mask & arch != 0)
}
