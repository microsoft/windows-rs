pub const ElementType: i32 = 0;
pub const TextType: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_ATTRIBUTE {
    pub Element: *mut WSDXML_ELEMENT,
    pub Next: *mut Self,
    pub Name: *mut WSDXML_NAME,
    pub Value: *mut u16,
}
impl Default for WSDXML_ATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_ELEMENT {
    pub Node: WSDXML_NODE,
    pub Name: *mut WSDXML_NAME,
    pub FirstAttribute: *mut WSDXML_ATTRIBUTE,
    pub FirstChild: *mut WSDXML_NODE,
    pub PrefixMappings: *mut WSDXML_PREFIX_MAPPING,
}
impl Default for WSDXML_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_ELEMENT_LIST {
    pub Next: *mut Self,
    pub Element: *mut WSDXML_ELEMENT,
}
impl Default for WSDXML_ELEMENT_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_NAME {
    pub Space: *mut WSDXML_NAMESPACE,
    pub LocalName: *mut u16,
}
impl Default for WSDXML_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_NAMESPACE {
    pub Uri: *const u16,
    pub PreferredPrefix: *const u16,
    pub Names: *mut WSDXML_NAME,
    pub NamesCount: u16,
    pub Encoding: u16,
}
impl Default for WSDXML_NAMESPACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_NODE {
    pub Type: i32,
    pub Parent: *mut WSDXML_ELEMENT,
    pub Next: *mut Self,
}
impl Default for WSDXML_NODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_PREFIX_MAPPING {
    pub Refs: u32,
    pub Next: *mut Self,
    pub Space: *mut WSDXML_NAMESPACE,
    pub Prefix: *mut u16,
}
impl Default for WSDXML_PREFIX_MAPPING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_TEXT {
    pub Node: WSDXML_NODE,
    pub Text: *mut u16,
}
impl Default for WSDXML_TEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSDXML_TYPE {
    pub Uri: *const u16,
    pub Table: *const u8,
}
impl Default for WSDXML_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
