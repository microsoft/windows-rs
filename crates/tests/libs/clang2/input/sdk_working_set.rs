
const _: () = {
    assert!(size_of::<PSAPI_WORKING_SET_BLOCK>() == size_of::<usize>());
    assert!(align_of::<PSAPI_WORKING_SET_BLOCK>() == align_of::<usize>());
};

fn main() {
    let mut block = PSAPI_WORKING_SET_BLOCK { Flags: 0 };
    let maximum = (1usize << (usize::BITS - 12)) - 1;
    let fields = unsafe { &mut block.Anonymous };
    fields.set_VirtualPage(maximum as ULONG_PTR);
    fields.set_Protection(31);
    fields.set_Shared(1);
    assert_eq!(fields.VirtualPage(), maximum as ULONG_PTR);
    assert_eq!(fields.Protection(), 31);
    assert_eq!(fields.Shared(), 1);
    assert_eq!(unsafe { block.Flags } as usize, (maximum << 12) | 0x11f);
}
