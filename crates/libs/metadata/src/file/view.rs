type Result<T> = std::result::Result<T, ()>;

pub trait View {
    fn view_as<T>(&self, offset: usize) -> Result<&T>;
    fn view_as_slice_of<T>(&self, offset: usize, len: usize) -> Result<&[T]>;
    fn copy_as<T: Copy>(&self, offset: usize) -> Result<T>;
    fn view_as_str(&self, offset: usize) -> Result<&[u8]>;
    fn is_proper_length<T>(&self, offset: usize) -> Result<()>;
    fn is_proper_length_and_alignment<T>(&self, offset: usize, count: usize) -> Result<*const T>;
}

impl View for [u8] {
    fn view_as<T>(&self, offset: usize) -> Result<&T> {
        unsafe { Ok(&*self.is_proper_length_and_alignment(offset, 1)?) }
    }

    fn view_as_slice_of<T>(&self, offset: usize, len: usize) -> Result<&[T]> {
        unsafe { Ok(std::slice::from_raw_parts(self.is_proper_length_and_alignment(offset, len)?, len)) }
    }

    fn copy_as<T>(&self, offset: usize) -> Result<T> {
        self.is_proper_length::<T>(offset)?;

        unsafe {
            let mut data = std::mem::MaybeUninit::zeroed().assume_init();
            std::ptr::copy_nonoverlapping(self[offset..].as_ptr(), &mut data as *mut T as *mut u8, std::mem::size_of::<T>());
            Ok(data)
        }
    }

    fn view_as_str(&self, offset: usize) -> Result<&[u8]> {
        let buffer = &self[offset..];
        let index = buffer.iter().position(|c| *c == b'\0').ok_or(())?;
        Ok(&self[offset..offset + index])
    }

    fn is_proper_length<T>(&self, offset: usize) -> Result<()> {
        if offset + std::mem::size_of::<T>() <= self.len() {
            Ok(())
        } else {
            Err(())
        }
    }

    fn is_proper_length_and_alignment<T>(&self, offset: usize, count: usize) -> Result<*const T> {
        self.is_proper_length::<T>(offset * count)?;
        let ptr = &self[offset] as *const u8 as *const T;

        if ptr.align_offset(std::mem::align_of::<T>()) == 0 {
            Ok(ptr)
        } else {
            Err(())
        }
    }
}
