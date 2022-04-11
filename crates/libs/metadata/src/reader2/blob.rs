use super::*;
use core::convert::TryInto;

pub struct Blob<'a> {
    scope: &'a Scope<'a>,
    file: usize,
    slice: &'a [u8],
}

impl<'a> Blob<'a> {
    pub fn new(scope: &'a Scope, file: usize, slice: &'a [u8]) -> Self {
        Self { scope, file, slice }
    }
    pub fn reader(&self) -> BlobReader {
        BlobReader::new(self)
    }
}

impl<'a> std::ops::Deref for Blob<'a> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.slice
    }
}

pub struct BlobReader<'a> {
    blob: &'a Blob<'a>,
    slice: &'a [u8],
}

impl<'a> BlobReader<'a> {
    pub fn read_usize(&mut self) -> usize {
        let (value, offset) = self.peek_usize();
        self.offset(offset);
        value
    }
    pub fn read_expected(&mut self, expected: usize) -> bool {
        let (value, offset) = self.peek_usize();
        if value == expected {
            self.offset(offset);
            true
        } else {
            false
        }
    }
    pub fn read_modifiers(&mut self) -> Vec<TypeDefOrRef> {
        let mut mods = vec![];
        loop {
            let (value, offset) = self.peek_usize();
            if value != 32 && value != 31 {
                break;
            } else {
                self.offset(offset);
                mods.push(self.read_type_def_or_ref())
            }
        }
        mods
    }
    pub fn read_str(&mut self) -> &str {
        let len = self.read_usize();
        let value = std::str::from_utf8(&self.slice[..len]).expect("Failed to read string in blob");
        self.offset(len);
        value
    }
    pub fn read_string(self) -> String {
        let slice = self.slice;
        if slice.as_ptr().align_offset(core::mem::align_of::<u16>()) > 0 {
            let slice = slice.chunks_exact(2).take(slice.len() / 2).map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap())).collect::<Vec<u16>>();
            String::from_utf16_lossy(&slice)
        } else {
            let slice = unsafe { core::slice::from_raw_parts(slice.as_ptr() as *const u16, slice.len() / 2) };
            String::from_utf16_lossy(slice)
        }
    }
    pub fn read_i8(&mut self) -> i8 {
        let value = i8::from_le_bytes(self.slice[..1].try_into().unwrap());
        self.offset(1);
        value
    }
    pub fn read_u8(&mut self) -> u8 {
        let value = u8::from_le_bytes(self.slice[..1].try_into().unwrap());
        self.offset(1);
        value
    }
    pub fn read_i16(&mut self) -> i16 {
        let value = i16::from_le_bytes(self.slice[..2].try_into().unwrap());
        self.offset(2);
        value
    }
    pub fn read_u16(&mut self) -> u16 {
        let value = u16::from_le_bytes(self.slice[..2].try_into().unwrap());
        self.offset(2);
        value
    }
    pub fn read_i32(&mut self) -> i32 {
        let value = i32::from_le_bytes(self.slice[..4].try_into().unwrap());
        self.offset(4);
        value
    }
    pub fn read_u32(&mut self) -> u32 {
        let value = u32::from_le_bytes(self.slice[..4].try_into().unwrap());
        self.offset(4);
        value
    }
    pub fn read_i64(&mut self) -> i64 {
        let value = i64::from_le_bytes(self.slice[..8].try_into().unwrap());
        self.offset(8);
        value
    }
    pub fn read_u64(&mut self) -> u64 {
        let value = u64::from_le_bytes(self.slice[..8].try_into().unwrap());
        self.offset(8);
        value
    }
    pub fn read_f32(&mut self) -> f32 {
        let value = f32::from_le_bytes(self.slice[..4].try_into().unwrap());
        self.offset(4);
        value
    }
    pub fn read_f64(&mut self) -> f64 {
        let value = f64::from_le_bytes(self.slice[..8].try_into().unwrap());
        self.offset(8);
        value
    }
    pub fn read_type(&mut self, enclosing: Option<&TypeDef>, generics: &[Type]) -> Type {
        let is_winrt_const_ref = self.read_modifiers().iter().any(|def| def.type_name() == TypeName::IsConst);
        let is_winrt_array_ref = self.read_expected(0x10);

        if self.read_expected(0x01) {
            return Type::Void;
        }

        let is_winrt_array = self.read_expected(0x1D);
        let mut pointers = 0;

        while self.read_expected(0x0f) {
            pointers += 1;
        }

        let mut kind = self.read_type_spec(enclosing, generics);

        if pointers > 0 {
            kind = Type::MutPtr((Box::new(kind), pointers));
        }

        if is_winrt_array {
            if is_winrt_array_ref {
                Type::WinrtArrayRef(Box::new(kind))
            } else {
                Type::WinrtArray(Box::new(kind))
            }
        } else if is_winrt_const_ref {
            Type::WinrtConstRef(Box::new(kind))
        } else {
            kind
        }
    }

    fn new(blob: &'a Blob) -> Self {
        Self {
            blob,
            slice : blob,
        }
    }
    fn offset(&mut self, offset: usize) {
        self.slice = &self.slice[offset..];
    }
    fn peek_usize(&self) -> (usize, usize) {
        let slice = self.slice;
        if slice[0] & 0x80 == 0 {
            (slice[0] as usize, 1)
        } else if slice[0] & 0xC0 == 0x80 {
            ((((slice[0] & 0x3F) as usize) << 8) | slice[1] as usize, 2)
        } else {
            ((((slice[0] & 0x1F) as usize) << 24) | (slice[1] as usize) << 16 | (slice[2] as usize) << 8 | slice[3] as usize, 4)
        }
    }
    fn read_type_def_or_ref(&mut self) -> TypeDefOrRef {
        TypeDefOrRef::decode(self.blob.scope, self.blob.file, self.read_usize())
    }
    fn read_type_spec(&mut self, enclosing: Option<&TypeDef>, generics: &[Type]) -> Type {
        let code = self.read_usize();

        if let Some(code) = Type::from_code(code) {
            return code;
        }

        match code {
            0x11 | 0x12 => self.read_type_def_or_ref().resolve(enclosing, generics),
            0x13 => generics.get(self.read_usize() as usize).unwrap_or(&Type::Void).clone(),
            0x14 => {
                let kind = self.read_type(enclosing, generics);
                let _rank = self.read_usize();
                let _bounds_count = self.read_usize();
                let bounds = self.read_usize();
                Type::Win32Array((Box::new(kind), bounds))
            }
            0x15 => {
                self.read_usize();

                let mut def = self.read_type_def_or_ref().resolve(None, &[]);
                let args = self.read_usize();

                for _ in 0..args {
                    def.1.push(self.read_type_spec(None, generics));
                }

                Type::TypeDef(def)
            }
            _ => unimplemented!(),
        }
    }
}
