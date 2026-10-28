fn accept(
    word: Word,
    signed: SignedWord,
    copy: Copy,
    pointer: Pointer,
    array: Array,
    status: Status,
    copy_status: CopyStatus,
    callback: Callback,
    holder: Holder,
) {
    let _: usize = word;
    let _: isize = signed;
    let _: usize = copy;
    let _: *mut usize = pointer;
    let _: [usize; 2] = array;
    let _: isize = status;
    let _: isize = copy_status;
    let _: Option<unsafe extern "system" fn(isize, u64) -> usize> = callback;
    let _: usize = holder.word;
    let _: isize = holder.signed_word;
    let _: u32 = holder.fixed32;
    let _: u64 = holder.fixed64;
    let _: usize = ONE;
    let _: isize = NEGATIVE;
}
