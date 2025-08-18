use super::*;

pub struct Blob<'a> {
    index: &'a TypeIndex,
    file: usize,
    slice: &'a [u8],
}

impl Drop for Blob<'_> {
    fn drop(&mut self) {
        debug_assert_eq!(self.len(), 0);
    }
}

impl std::fmt::Debug for Blob<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.slice)
    }
}

impl std::ops::Deref for Blob<'_> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.slice
    }
}

impl<'a> Blob<'a> {
    pub fn new(index: &'a TypeIndex, file: usize, slice: &'a [u8]) -> Self {
        Self { index, file, slice }
    }

    fn peek(&self) -> (usize, usize) {
        if self[0] & 0x80 == 0 {
            (self[0] as usize, 1)
        } else if self[0] & 0xC0 == 0x80 {
            ((((self[0] & 0x3F) as usize) << 8) | self[1] as usize, 2)
        } else {
            (
                (((self[0] & 0x1F) as usize) << 24)
                    | ((self[1] as usize) << 16)
                    | ((self[2] as usize) << 8)
                    | self[3] as usize,
                4,
            )
        }
    }

    pub fn decode<D: Decode<'a>>(&mut self) -> D {
        D::decode(self.index, self.file, self.read_compressed())
    }

    pub fn try_read(&mut self, expected: usize) -> bool {
        let (value, offset) = self.peek();
        if value == expected {
            self.offset(offset);
            true
        } else {
            false
        }
    }

    pub fn read_modifiers(&mut self) -> Vec<TypeDefOrRef<'a>> {
        let mut mods = vec![];
        loop {
            let (value, offset) = self.peek();
            if value != ELEMENT_TYPE_CMOD_OPT as usize && value != ELEMENT_TYPE_CMOD_REQD as usize {
                break;
            } else {
                self.offset(offset);
                mods.push(TypeDefOrRef::decode(
                    self.index,
                    self.file,
                    self.read_compressed(),
                ))
            }
        }
        mods
    }

    pub fn read_method_signature(&mut self, generics: &[Type]) -> Signature {
        let flags = MethodCallAttributes(self.read_u8());
        let param_count = self.read_compressed();
        let return_type = self.read_type_signature(generics);

        let mut types = vec![];

        for _ in 0..param_count {
            types.push(self.read_type_signature(generics));
        }

        Signature {
            flags,
            return_type,
            types,
        }
    }

    // Used to parse field and methods type signatures
    pub fn read_type_signature(&mut self, generics: &[Type]) -> Type {
        let is_const = self.read_modifiers().iter().any(|def| {
            def.namespace() == "System.Runtime.CompilerServices" && def.name() == "IsConst"
        });

        let is_ref = self.try_read(ELEMENT_TYPE_BYREF as usize);

        if self.try_read(ELEMENT_TYPE_VOID as usize) {
            return Type::Void;
        }

        let is_array = self.try_read(ELEMENT_TYPE_SZARRAY as usize);

        let mut pointers = 0;

        while self.try_read(ELEMENT_TYPE_PTR as usize) {
            pointers += 1;
        }

        let ty = self.read_type_code(generics);

        // TODO: why don't we just use IsConst to decide whether pointers are const?
        if pointers > 0 {
            Type::PtrMut(Box::new(ty), pointers)
        } else if is_const {
            Type::ConstRef(Box::new(ty))
        } else if is_array {
            if is_ref {
                Type::ArrayRef(Box::new(ty))
            } else {
                Type::Array(Box::new(ty))
            }
        } else {
            ty
        }
    }

    pub fn read_type_code(&mut self, generics: &[Type]) -> Type {
        match self.read_u8() {
            ELEMENT_TYPE_VOID => Type::Void,
            ELEMENT_TYPE_BOOLEAN => Type::Bool,
            ELEMENT_TYPE_CHAR => Type::Char,
            ELEMENT_TYPE_I1 => Type::I8,
            ELEMENT_TYPE_U1 => Type::U8,
            ELEMENT_TYPE_I2 => Type::I16,
            ELEMENT_TYPE_U2 => Type::U16,
            ELEMENT_TYPE_I4 => Type::I32,
            ELEMENT_TYPE_U4 => Type::U32,
            ELEMENT_TYPE_I8 => Type::I64,
            ELEMENT_TYPE_U8 => Type::U64,
            ELEMENT_TYPE_R4 => Type::F32,
            ELEMENT_TYPE_R8 => Type::F64,
            ELEMENT_TYPE_I => Type::ISize,
            ELEMENT_TYPE_U => Type::USize,
            ELEMENT_TYPE_STRING => Type::String,
            ELEMENT_TYPE_OBJECT => Type::Object,
            ELEMENT_TYPE_VALUETYPE | ELEMENT_TYPE_CLASS => {
                self.decode::<TypeDefOrRef>().ty(generics)
            }
            ELEMENT_TYPE_VAR => generics[self.read_compressed()].clone(),
            ELEMENT_TYPE_ARRAY => {
                // See II.23.2.13 ArrayShape
                let ty = self.read_type_signature(generics);
                let rank = self.read_compressed();
                debug_assert_eq!(rank, 1);
                let num_sizes = self.read_compressed();
                debug_assert_eq!(num_sizes, 1);
                let size = self.read_compressed();
                let num_lo_bounds = self.read_compressed();
                debug_assert!(num_lo_bounds == 0 || num_lo_bounds == 1);

                for _ in 0..num_lo_bounds {
                    let lo_bounds = self.read_compressed();
                    debug_assert_eq!(lo_bounds, 0);
                }

                Type::ArrayFixed(Box::new(ty), size)
            }
            ELEMENT_TYPE_GENERICINST => {
                let type_code = self.read_u8();

                debug_assert!(matches!(
                    type_code,
                    ELEMENT_TYPE_VALUETYPE | ELEMENT_TYPE_CLASS
                ));

                let ty = self.decode::<TypeDefOrRef>();
                let mut ty_generics = vec![];

                for _ in 0..self.read_compressed() {
                    ty_generics.push(self.read_type_code(generics));
                }

                Type::Name(TypeName {
                    namespace: ty.namespace().to_string(),
                    name: ty.name().to_string(),
                    generics: ty_generics,
                })
            }
            0x55 => Type::AttributeEnum,
            rest => panic!("{rest:?}"),
        }
    }

    pub fn read_compressed(&mut self) -> usize {
        let (value, offset) = self.peek();
        self.offset(offset);
        value
    }

    pub fn read_utf8(&mut self) -> String {
        let len = self.read_compressed();
        let value = unsafe { std::str::from_utf8_unchecked(&self.slice[..len]) };
        self.offset(len);
        value.to_string()
    }

    pub fn read_utf16(&mut self) -> String {
        let slice = self.slice;

        let value = if slice.as_ptr().align_offset(align_of::<u16>()) > 0 {
            let slice = slice
                .chunks_exact(2)
                .take(slice.len() / 2)
                .map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap()))
                .collect::<Vec<u16>>();

            String::from_utf16_lossy(&slice)
        } else {
            let slice = unsafe {
                std::slice::from_raw_parts(slice.as_ptr() as *const u16, slice.len() / 2)
            };

            String::from_utf16_lossy(slice)
        };

        self.offset(slice.len());
        value
    }

    pub fn read_bool(&mut self) -> bool {
        // A bool is specified as "a single byte with value 0 (false) or 1 (true)".
        match self.read_u8() {
            0 => false,
            1 => true,
            _ => panic!(),
        }
    }

    pub fn read_i8(&mut self) -> i8 {
        let value = i8::from_le_bytes(self[..1].try_into().unwrap());
        self.offset(1);
        value
    }

    pub fn read_u8(&mut self) -> u8 {
        let value = u8::from_le_bytes(self[..1].try_into().unwrap());
        self.offset(1);
        value
    }

    pub fn read_i16(&mut self) -> i16 {
        let value = i16::from_le_bytes(self[..2].try_into().unwrap());
        self.offset(2);
        value
    }

    pub fn read_u16(&mut self) -> u16 {
        let value = u16::from_le_bytes(self[..2].try_into().unwrap());
        self.offset(2);
        value
    }

    pub fn read_i32(&mut self) -> i32 {
        let value = i32::from_le_bytes(self[..4].try_into().unwrap());
        self.offset(4);
        value
    }

    pub fn read_u32(&mut self) -> u32 {
        let value = u32::from_le_bytes(self[..4].try_into().unwrap());
        self.offset(4);
        value
    }

    pub fn read_i64(&mut self) -> i64 {
        let value = i64::from_le_bytes(self[..8].try_into().unwrap());
        self.offset(8);
        value
    }

    pub fn read_u64(&mut self) -> u64 {
        let value = u64::from_le_bytes(self[..8].try_into().unwrap());
        self.offset(8);
        value
    }

    pub fn read_f32(&mut self) -> f32 {
        let value = f32::from_le_bytes(self[..4].try_into().unwrap());
        self.offset(4);
        value
    }

    pub fn read_f64(&mut self) -> f64 {
        let value = f64::from_le_bytes(self[..8].try_into().unwrap());
        self.offset(8);
        value
    }

    fn offset(&mut self, offset: usize) {
        self.slice = &self.slice[offset..];
    }
}
