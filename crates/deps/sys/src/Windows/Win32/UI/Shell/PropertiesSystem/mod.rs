#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn ClearPropVariantArray();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn ClearVariantArray();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromBooleanVector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromBuffer();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromCLSID();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromDoubleVector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromFileTime();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromFileTimeVector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromGUIDAsString();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromInt16Vector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromInt32Vector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromInt64Vector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromPropVariantVectorElem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromResource();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_Common"))]
    pub fn InitPropVariantFromStrRet();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromStringAsVector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromStringVector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromUInt16Vector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromUInt32Vector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantFromUInt64Vector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn InitPropVariantVectorFromPropVariant();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromBooleanArray();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromBuffer();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromDoubleArray();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromFileTime();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromFileTimeArray();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromGUIDAsString();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromInt16Array();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromInt32Array();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromInt64Array();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromResource();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
    pub fn InitVariantFromStrRet();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromStringArray();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromUInt16Array();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromUInt32Array();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromUInt64Array();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn InitVariantFromVariantArrayElem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSCoerceToCanonicalValue();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub fn PSCreateAdapterFromPropertyStore();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub fn PSCreateDelayedMultiplexPropertyStore();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub fn PSCreateMemoryPropertyStore();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub fn PSCreateMultiplexPropertyStore();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSCreatePropertyChangeArray();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub fn PSCreatePropertyStoreFromObject();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn PSCreatePropertyStoreFromPropertySetStorage();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSCreateSimplePropertyChange();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub fn PSEnumeratePropertyDescriptions();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSFormatForDisplay();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSFormatForDisplayAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSFormatPropertyValue();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSGetImageReferenceForValue();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSGetItemPropertyHandler();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSGetItemPropertyHandlerWithCreateObject();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSGetNameFromPropertyKey();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSGetNamedPropertyFromPropertyStorage();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub fn PSGetPropertyDescription();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSGetPropertyDescriptionByName();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSGetPropertyDescriptionListFromString();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSGetPropertyFromPropertyStorage();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSGetPropertyKeyFromName();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub fn PSGetPropertySystem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSGetPropertyValue();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSLookupPropertyHandlerCLSID();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_Delete();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadBOOL();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadBSTR();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadDWORD();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadGUID();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadInt();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadLONG();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadPOINTL();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadPOINTS();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadPropertyKey();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadRECTL();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadSHORT();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadStr();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadStrAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadStream();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub fn PSPropertyBag_ReadType();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadULONGLONG();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_ReadUnknown();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteBOOL();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteBSTR();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteDWORD();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteGUID();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteInt();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteLONG();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WritePOINTL();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WritePOINTS();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WritePropertyKey();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteRECTL();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteSHORT();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteStr();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteStream();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteULONGLONG();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSPropertyBag_WriteUnknown();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSPropertyKeyFromString();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`*"]
    pub fn PSRefreshPropertySchema();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSRegisterPropertySchema();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PSSetPropertyValue();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSStringFromPropertyKey();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PSUnregisterPropertySchema();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PifMgr_CloseProperties();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PifMgr_GetProperties();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PifMgr_OpenProperties();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PifMgr_SetProperties();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantChangeType();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantCompareEx();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetBooleanElem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetDoubleElem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetElementCount();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetFileTimeElem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetInt16Elem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetInt32Elem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetInt64Elem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetStringElem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetUInt16Elem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetUInt32Elem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantGetUInt64Elem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToBSTR();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToBoolean();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToBooleanVector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToBooleanVectorAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToBooleanWithDefault();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToBuffer();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToDouble();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToDoubleVector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToDoubleVectorAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToDoubleWithDefault();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToFileTime();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToFileTimeVector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToFileTimeVectorAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToGUID();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt16();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt16Vector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt16VectorAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt16WithDefault();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt32();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt32Vector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt32VectorAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt32WithDefault();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt64();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt64Vector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt64VectorAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToInt64WithDefault();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_Common"))]
    pub fn PropVariantToStrRet();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToString();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToStringAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToStringVector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToStringVectorAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToStringWithDefault();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt16();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt16Vector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt16VectorAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt16WithDefault();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt32();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt32Vector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt32VectorAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt32WithDefault();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt64();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt64Vector();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt64VectorAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToUInt64WithDefault();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub fn PropVariantToVariant();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn PropVariantToWinRTPropertyValue();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAddDefaultPropertiesByExt();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetPropertyStoreForWindow();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHGetPropertyStoreFromIDList();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHGetPropertyStoreFromParsingName();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn SHPropStgCreate();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn SHPropStgReadMultiple();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn SHPropStgWriteMultiple();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantCompare();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetBooleanElem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetDoubleElem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetElementCount();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetInt16Elem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetInt32Elem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetInt64Elem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetStringElem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetUInt16Elem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetUInt32Elem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantGetUInt64Elem();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToBoolean();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToBooleanArray();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToBooleanArrayAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToBooleanWithDefault();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToBuffer();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToDosDateTime();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToDouble();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToDoubleArray();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToDoubleArrayAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToDoubleWithDefault();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToFileTime();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToGUID();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt16();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt16Array();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt16ArrayAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt16WithDefault();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt32();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt32Array();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt32ArrayAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt32WithDefault();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt64();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt64Array();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt64ArrayAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToInt64WithDefault();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole"))]
    pub fn VariantToPropVariant();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
    pub fn VariantToStrRet();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToString();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToStringAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToStringArray();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToStringArrayAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToStringWithDefault();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt16();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt16Array();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt16ArrayAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt16WithDefault();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt32();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt32Array();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt32ArrayAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt32WithDefault();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt64();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt64Array();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt64ArrayAlloc();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn VariantToUInt64WithDefault();
    #[doc = "*Required features: `Win32_UI_Shell_PropertiesSystem`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn WinRTPropertyValueToPropVariant();
}
