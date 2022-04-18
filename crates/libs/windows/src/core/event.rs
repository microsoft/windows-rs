use super::*;
use bindings::*;
use std::sync::*;

struct Event<T: RuntimeType> {
    swap: Mutex<()>,
    change: Mutex<()>,
    delegates: Delegates<T>,
}

impl<T: RuntimeType> Default for Event<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: RuntimeType> Event<T> {
    pub fn new() -> Self {
        Self { delegates: Delegates::new(), ..Default::default() }
    }
    pub fn add(&mut self, delegate: &T) -> Result<i64> {
        let mut token = 0;
        let mut _temp_delegates = Delegates::<T>::new();
        {
            let _ = self.change.lock().unwrap();
            let mut new_delegates = Delegates::with_len(self.delegates.len() + 1)?;
            let new_slice = new_delegates.as_mut_slice();
            new_slice[..self.delegates.len()].clone_from_slice(&self.delegates.as_slice());
            new_slice[self.delegates.len()] = Some(delegate.clone()); // TODO: need to agile wrap this one
            token = to_token(delegate); // TODO: make sure this is the agile-wrapped delegate that is being tokenized

            let _ = self.swap.lock().unwrap();
            _temp_delegates = self.delegates.swap(new_delegates);
        }
        Ok(token)
    }
    pub fn remove(&mut self, token: i64) -> Result<()> {
        todo!()
    }
    pub fn clear(&mut self) {}
    pub fn call(&mut self) -> Result<()> {
        todo!()
    }
}

struct Delegates<T: RuntimeType>(*mut Header, std::marker::PhantomData<T>);

impl<T: RuntimeType> Default for Delegates<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: RuntimeType> Delegates<T> {
    fn new() -> Self {
        Self(std::ptr::null_mut(), std::marker::PhantomData)
    }
    fn with_len(len: usize) -> Result<Self> {
        Ok(Self(Header::new(len)?, std::marker::PhantomData))
    }
    fn swap(&mut self, other: Self) -> Self {
        unsafe { std::ptr::swap(self.0, other.0) };
        other
    }
    fn is_empty(&self) -> bool {
        self.0.is_null()
    }
    fn len(&self) -> usize {
        if self.is_empty() {
            0
        } else {
            unsafe { (*self.0).len }
        }
    }
    fn as_slice(&self) -> &[Option<T>] {
        if self.is_empty() {
            &[]
        } else {
            unsafe {
                let slice = (*self.0).as_slice();
                std::slice::from_raw_parts(slice.as_ptr() as _, slice.len())
            }
        }
    }
    fn as_mut_slice(&mut self) -> &mut [Option<T>] {
        if self.is_empty() {
            &mut []
        } else {
            unsafe {
                let slice = (*self.0).as_mut_slice();
                std::slice::from_raw_parts_mut(slice.as_ptr() as _, slice.len())
            }
        }
    }
}

impl<T: RuntimeType> Clone for Delegates<T> {
    fn clone(&self) -> Self {
        if !self.is_empty() {
            unsafe { (*self.0).count.add_ref() };
        }
        Self(self.0, std::marker::PhantomData)
    }
}

impl<T: RuntimeType> Drop for Delegates<T> {
    fn drop(&mut self) {
        unsafe {
            if !self.is_empty() && (*self.0).count.release() == 0 {
                std::ptr::drop_in_place(self.as_mut_slice());
                heap_free(self.0 as _)
            }
        }
    }
}

#[repr(C)]
struct Header {
    count: RefCount,
    len: usize,
}

impl Header {
    fn new(len: usize) -> Result<*mut Header> {
        if len == 0 {
            Ok(std::ptr::null_mut())
        } else {
            let alloc_size = std::mem::size_of::<Header>() + len * std::mem::size_of::<usize>();
            let header = heap_alloc(alloc_size)? as *mut Header;
            unsafe {
                (*header).count = RefCount::new(1);
                (*header).len = len;
                std::ptr::write_bytes(header.add(1) as *mut usize, 0, len);
            }
            Ok(header)
        }
    }
    fn as_slice(&self) -> &[usize] {
        unsafe { std::slice::from_raw_parts::<usize>((self as *const Self).add(1) as *const usize, self.len) }
    }
    fn as_mut_slice(&mut self) -> &mut [usize] {
        unsafe { std::slice::from_raw_parts_mut::<usize>((self as *mut Self).add(1) as *mut usize, self.len) }
    }
}

fn to_token<T: RuntimeType>(delegate: &T) -> i64 {
    unsafe { EncodePointer(std::mem::transmute_copy(delegate)) as _ }
}
