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
        let mut _temp_delegates = Delegates::new();
        {
            let _ = self.change.lock().unwrap();
            let mut new_delegates = Delegates::with_capacity(self.delegates.len() + 1)?;
            for delegate in self.delegates.as_slice() {
                new_delegates.push(delegate);
            }
            new_delegates.push(delegate);// TODO: need to agile wrap this one
            token = to_token(delegate); // TODO: make sure this is the agile-wrapped delegate that is being tokenized

            let _ = self.swap.lock().unwrap();
            _temp_delegates = self.delegates.swap(new_delegates);
        }
        Ok(token)
    }
    pub fn remove(&mut self, token: i64) -> Result<()> {
        let mut _temp_delegates = Delegates::new();
        {
            let _ = self.change.lock().unwrap();
            if self.delegates.is_empty() {
                return Ok(());
            }
            let mut capacity = self.delegates.len() - 1;
            let mut new_delegates = Delegates::new();
            let mut removed = false;
            if capacity == 0 {
                if to_token(&self.delegates.as_slice()[0]) == token {
                    removed = true;
                }
            } else {
                new_delegates = Delegates::with_capacity(capacity)?;
                for delegate in self.delegates.as_slice() {
                    if !removed && to_token(delegate) == token {
                        removed = true;
                        continue;
                    }
                    if capacity == 0 {
                        debug_assert!(!removed);
                        break;
                    }
                    new_delegates.push(delegate);
                    capacity -= 1;
                }
            }
            if removed {
                let _ = self.swap.lock().unwrap();
                _temp_delegates = self.delegates.swap(new_delegates);
            }
        }
        Ok(())
    }
    pub fn clear(&mut self) {
        let mut _temp_delegates = Delegates::new();
        {
            let _ = self.change.lock().unwrap();
            if self.delegates.is_empty() {
                return;
            }
            let _ = self.swap.lock().unwrap();
            _temp_delegates = self.delegates.swap(Delegates::new());
        }
    }
    pub fn call(&mut self) -> Result<()> {
        todo!()
    }
}

struct Delegates<T: RuntimeType>{
    buffer: *mut Buffer,
    len: usize,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: RuntimeType> Default for Delegates<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: RuntimeType> Delegates<T> {
    fn new() -> Self {
        Self{buffer: std::ptr::null_mut(), len: 0, _phantom: std::marker::PhantomData}
    }
    fn with_capacity(capacity: usize) -> Result<Self> {
        Ok(Self{ buffer: Buffer::new(capacity)?, len: 0, _phantom: std::marker::PhantomData})
    }
    fn swap(&mut self, other: Self) -> Self {
        unsafe { std::ptr::swap(self.buffer, other.buffer) };
        other
    }
    fn is_empty(&self) -> bool {
        self.len == 0
    }
    fn len(&self) -> usize {
        self.len
    }
    fn push(&mut self, delegate: &T) {
        unsafe {
        debug_assert!(self.len < (*self.buffer).len);
        std::ptr::write((*self.buffer).as_mut_ptr().add(self.len) as _, delegate.clone());
        self.len += 1;
        }
    }
    fn as_slice(&self) -> &[T] {
        if self.is_empty() {
            &[]
        } else {
            unsafe {
                std::slice::from_raw_parts((*self.buffer).as_ptr() as _, self.len)
            }
        }
    }
    fn as_mut_slice(&mut self) -> &mut [Option<T>] {
        if self.is_empty() {
            &mut []
        } else {
            unsafe {
                std::slice::from_raw_parts_mut((*self.buffer).as_mut_ptr() as _, self.len)
            }
        }
    }
}

impl<T: RuntimeType> Clone for Delegates<T> {
    fn clone(&self) -> Self {
        if !self.is_empty() {
            unsafe { (*self.buffer).count.add_ref() };
        }
        Self{buffer: self.buffer, len:self.len, _phantom: std::marker::PhantomData}
    }
}

impl<T: RuntimeType> Drop for Delegates<T> {
    fn drop(&mut self) {
        unsafe {
            if !self.is_empty() && (*self.buffer).count.release() == 0 {
                std::ptr::drop_in_place(self.as_mut_slice());
                heap_free(self.buffer as _)
            }
        }
    }
}

#[repr(C)]
struct Buffer {
    count: RefCount,
    len: usize,
}

impl Buffer {
    fn new(len: usize) -> Result<*mut Buffer> {
        if len == 0 {
            Ok(std::ptr::null_mut())
        } else {
            let alloc_size = std::mem::size_of::<Buffer>() + len * std::mem::size_of::<usize>();
            let header = heap_alloc(alloc_size)? as *mut Buffer;
            unsafe {
                (*header).count = RefCount::new(1);
                (*header).len = len;
            }
            Ok(header)
        }
    }
    fn as_ptr(&self) -> *const usize {
        unsafe { (self as *const Self).add(1) as *const _ }
    }
    fn as_mut_ptr(&mut self) -> *mut usize {
        unsafe { (self as *mut Self).add(1) as *mut _ }
    }
}

fn to_token<T: RuntimeType>(delegate: &T) -> i64 {
    unsafe { EncodePointer(std::mem::transmute_copy(delegate)) as _ }
}
