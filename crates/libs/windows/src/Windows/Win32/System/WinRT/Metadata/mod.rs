pub const ASSEMBLY_METADATA_TYPE: windows_core::PCSTR = windows_core::s!("System.Reflection.AssemblyMetadataAttribute");
pub const ASSEMBLY_METADATA_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Reflection.AssemblyMetadataAttribute");
pub const CLSID_CLR_v1_MetaData: windows_core::GUID = windows_core::GUID::from_u128(0x005023ca_72b1_11d3_9fc4_00c04f79a0a3);
pub const CLSID_CLR_v2_MetaData: windows_core::GUID = windows_core::GUID::from_u128(0xefea471a_44fd_4862_9292_0c58d46e1f3a);
pub const CLSID_Cor: windows_core::GUID = windows_core::GUID::from_u128(0xbee00010_ee77_11d0_a015_00c04fbbb884);
pub const CLSID_CorMetaDataDispenser: windows_core::GUID = windows_core::GUID::from_u128(0xe5cb7a31_7512_11d2_89ce_0080c792e5d8);
pub const CLSID_CorMetaDataDispenserReg: windows_core::GUID = windows_core::GUID::from_u128(0x435755ff_7397_11d2_9771_00a0c9b4d50c);
pub const CLSID_CorMetaDataDispenserRuntime: windows_core::GUID = windows_core::GUID::from_u128(0x1ec2de53_75cc_11d2_9775_00a0c9b4d50c);
pub const CLSID_CorMetaDataReg: windows_core::GUID = windows_core::GUID::from_u128(0x87f3a1f5_7397_11d2_9771_00a0c9b4d50c);
pub const CMOD_CALLCONV_NAMESPACE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices");
pub const CMOD_CALLCONV_NAMESPACE_OLD: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices");
pub const CMOD_CALLCONV_NAME_CDECL: windows_core::PCSTR = windows_core::s!("CallConvCdecl");
pub const CMOD_CALLCONV_NAME_FASTCALL: windows_core::PCSTR = windows_core::s!("CallConvFastcall");
pub const CMOD_CALLCONV_NAME_STDCALL: windows_core::PCSTR = windows_core::s!("CallConvStdcall");
pub const CMOD_CALLCONV_NAME_THISCALL: windows_core::PCSTR = windows_core::s!("CallConvThiscall");
pub const COINITCOR_DEFAULT: COINITICOR = 0i32;
pub const COINITEE_DEFAULT: COINITIEE = 0i32;
pub const COINITEE_DLL: COINITIEE = 1i32;
pub const COINITEE_MAIN: COINITIEE = 2i32;
pub const COMPILATIONRELAXATIONS_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.CompilationRelaxationsAttribute");
pub const COMPILATIONRELAXATIONS_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.CompilationRelaxationsAttribute");
pub const COR_BASE_SECURITY_ATTRIBUTE_CLASS: windows_core::PCWSTR = windows_core::w!("System.Security.Permissions.SecurityAttribute");
pub const COR_BASE_SECURITY_ATTRIBUTE_CLASS_ANSI: windows_core::PCSTR = windows_core::s!("System.Security.Permissions.SecurityAttribute");
pub const COR_CCTOR_METHOD_NAME: windows_core::PCSTR = windows_core::s!(".cctor");
pub const COR_CCTOR_METHOD_NAME_W: windows_core::PCWSTR = windows_core::w!(".cctor");
pub const COR_COMPILERSERVICE_DISCARDABLEATTRIBUTE: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.DiscardableAttribute");
pub const COR_COMPILERSERVICE_DISCARDABLEATTRIBUTE_ASNI: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.DiscardableAttribute");
pub const COR_CTOR_METHOD_NAME: windows_core::PCSTR = windows_core::s!(".ctor");
pub const COR_CTOR_METHOD_NAME_W: windows_core::PCWSTR = windows_core::w!(".ctor");
pub const COR_DELETED_NAME_A: windows_core::PCSTR = windows_core::s!("_Deleted");
pub const COR_DELETED_NAME_W: windows_core::PCWSTR = windows_core::w!("_Deleted");
pub const COR_ENUM_FIELD_NAME: windows_core::PCSTR = windows_core::s!("value__");
pub const COR_ENUM_FIELD_NAME_W: windows_core::PCWSTR = windows_core::w!("value__");
pub const COR_E_AMBIGUOUSMATCH: windows_core::HRESULT = 0x8000211D_u32 as _;
pub const COR_E_ARGUMENT: i32 = -2147024809i32;
pub const COR_E_BADIMAGEFORMAT: windows_core::HRESULT = 0x8007000B_u32 as _;
pub const COR_E_DIVIDEBYZERO: windows_core::HRESULT = 0x80020012_u32 as _;
pub const COR_E_INVALIDCAST: i32 = -2147467262i32;
pub const COR_E_NULLREFERENCE: i32 = -2147467261i32;
pub const COR_E_OUTOFMEMORY: i32 = -2147024882i32;
pub const COR_E_TARGETPARAMCOUNT: windows_core::HRESULT = 0x8002000E_u32 as _;
pub const COR_E_UNAUTHORIZEDACCESS: i32 = -2147024891i32;
pub const COR_ILEXCEPTION_CLAUSE_DEPRECATED: CorExceptionFlag = 0i32;
pub const COR_ILEXCEPTION_CLAUSE_DUPLICATED: CorExceptionFlag = 8i32;
pub const COR_ILEXCEPTION_CLAUSE_FAULT: CorExceptionFlag = 4i32;
pub const COR_ILEXCEPTION_CLAUSE_FILTER: CorExceptionFlag = 1i32;
pub const COR_ILEXCEPTION_CLAUSE_FINALLY: CorExceptionFlag = 2i32;
pub const COR_ILEXCEPTION_CLAUSE_NONE: CorExceptionFlag = 0i32;
pub const COR_ILEXCEPTION_CLAUSE_OFFSETLEN: CorExceptionFlag = 0i32;
pub const COR_NATIVE_LINK_CUSTOM_VALUE: windows_core::PCWSTR = windows_core::w!("COMPLUS_NativeLink");
pub const COR_NATIVE_LINK_CUSTOM_VALUE_ANSI: windows_core::PCSTR = windows_core::s!("COMPLUS_NativeLink");
pub const COR_NATIVE_LINK_CUSTOM_VALUE_CC: u32 = 18u32;
pub const COR_REQUIRES_SECOBJ_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("System.Security.DynamicSecurityMethodAttribute");
pub const COR_REQUIRES_SECOBJ_ATTRIBUTE_ANSI: windows_core::PCSTR = windows_core::s!("System.Security.DynamicSecurityMethodAttribute");
pub const COR_SUPPRESS_UNMANAGED_CODE_CHECK_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("System.Security.SuppressUnmanagedCodeSecurityAttribute");
pub const COR_SUPPRESS_UNMANAGED_CODE_CHECK_ATTRIBUTE_ANSI: windows_core::PCSTR = windows_core::s!("System.Security.SuppressUnmanagedCodeSecurityAttribute");
pub const COR_UNVER_CODE_ATTRIBUTE: windows_core::PCWSTR = windows_core::w!("System.Security.UnverifiableCodeAttribute");
pub const COR_UNVER_CODE_ATTRIBUTE_ANSI: windows_core::PCSTR = windows_core::s!("System.Security.UnverifiableCodeAttribute");
pub const COR_VTABLEGAP_NAME_A: windows_core::PCSTR = windows_core::s!("_VtblGap");
pub const COR_VTABLEGAP_NAME_W: windows_core::PCWSTR = windows_core::w!("_VtblGap");
pub const COUNINITEE_DEFAULT: COUNINITIEE = 0i32;
pub const COUNINITEE_DLL: COUNINITIEE = 1i32;
pub const CompilationRelaxations_NoStringInterning: CompilationRelaxationsEnum = 8i32;
pub const CorILMethod_CompressedIL: CorILMethodFlags = 64i32;
pub const CorILMethod_FatFormat: CorILMethodFlags = 3i32;
pub const CorILMethod_FormatMask: CorILMethodFlags = 7i32;
pub const CorILMethod_FormatShift: CorILMethodFlags = 3i32;
pub const CorILMethod_InitLocals: CorILMethodFlags = 16i32;
pub const CorILMethod_MoreSects: CorILMethodFlags = 8i32;
pub const CorILMethod_Sect_EHTable: CorILMethodSect = 1i32;
pub const CorILMethod_Sect_FatFormat: CorILMethodSect = 64i32;
pub const CorILMethod_Sect_KindMask: CorILMethodSect = 63i32;
pub const CorILMethod_Sect_MoreSects: CorILMethodSect = 128i32;
pub const CorILMethod_Sect_OptILTable: CorILMethodSect = 2i32;
pub const CorILMethod_Sect_Reserved: CorILMethodSect = 0i32;
pub const CorILMethod_SmallFormat: CorILMethodFlags = 0i32;
pub const CorILMethod_TinyFormat: CorILMethodFlags = 2i32;
pub const CorILMethod_TinyFormat1: CorILMethodFlags = 6i32;
pub const DEFAULTDEPENDENCY_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.DefaultDependencyAttribute");
pub const DEFAULTDEPENDENCY_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.DefaultDependencyAttribute");
pub const DEFAULTDOMAIN_LOADEROPTIMIZATION_TYPE: windows_core::PCSTR = windows_core::s!("System.LoaderOptimizationAttribute");
pub const DEFAULTDOMAIN_LOADEROPTIMIZATION_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.LoaderOptimizationAttribute");
pub const DEFAULTDOMAIN_MTA_TYPE: windows_core::PCSTR = windows_core::s!("System.MTAThreadAttribute");
pub const DEFAULTDOMAIN_MTA_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.MTAThreadAttribute");
pub const DEFAULTDOMAIN_STA_TYPE: windows_core::PCSTR = windows_core::s!("System.STAThreadAttribute");
pub const DEFAULTDOMAIN_STA_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.STAThreadAttribute");
pub const DEPENDENCY_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.DependencyAttribute");
pub const DEPENDENCY_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.DependencyAttribute");
pub const DESCR_GROUP_METHODDEF: i32 = 0i32;
pub const DESCR_GROUP_METHODIMPL: i32 = 1i32;
pub const DISABLED_PRIVATE_REFLECTION_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.DisablePrivateReflectionAttribute");
pub const DISABLED_PRIVATE_REFLECTION_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.DisablePrivateReflectionAttribute");
pub const DropMemberRefCAs: MergeFlags = 2i32;
pub const ELEMENT_TYPE_ARRAY: CorElementType = 20u8;
pub const ELEMENT_TYPE_BOOLEAN: CorElementType = 2u8;
pub const ELEMENT_TYPE_BYREF: CorElementType = 16u8;
pub const ELEMENT_TYPE_CHAR: CorElementType = 3u8;
pub const ELEMENT_TYPE_CLASS: CorElementType = 18u8;
pub const ELEMENT_TYPE_CMOD_OPT: CorElementType = 32u8;
pub const ELEMENT_TYPE_CMOD_REQD: CorElementType = 31u8;
pub const ELEMENT_TYPE_END: CorElementType = 0u8;
pub const ELEMENT_TYPE_FNPTR: CorElementType = 27u8;
pub const ELEMENT_TYPE_GENERICINST: CorElementType = 21u8;
pub const ELEMENT_TYPE_I: CorElementType = 24u8;
pub const ELEMENT_TYPE_I1: CorElementType = 4u8;
pub const ELEMENT_TYPE_I2: CorElementType = 6u8;
pub const ELEMENT_TYPE_I4: CorElementType = 8u8;
pub const ELEMENT_TYPE_I8: CorElementType = 10u8;
pub const ELEMENT_TYPE_INTERNAL: CorElementType = 33u8;
pub const ELEMENT_TYPE_MAX: CorElementType = 34u8;
pub const ELEMENT_TYPE_MODIFIER: CorElementType = 64u8;
pub const ELEMENT_TYPE_MVAR: CorElementType = 30u8;
pub const ELEMENT_TYPE_OBJECT: CorElementType = 28u8;
pub const ELEMENT_TYPE_PINNED: CorElementType = 69u8;
pub const ELEMENT_TYPE_PTR: CorElementType = 15u8;
pub const ELEMENT_TYPE_R4: CorElementType = 12u8;
pub const ELEMENT_TYPE_R8: CorElementType = 13u8;
pub const ELEMENT_TYPE_SENTINEL: CorElementType = 65u8;
pub const ELEMENT_TYPE_STRING: CorElementType = 14u8;
pub const ELEMENT_TYPE_SZARRAY: CorElementType = 29u8;
pub const ELEMENT_TYPE_TYPEDBYREF: CorElementType = 22u8;
pub const ELEMENT_TYPE_U: CorElementType = 25u8;
pub const ELEMENT_TYPE_U1: CorElementType = 5u8;
pub const ELEMENT_TYPE_U2: CorElementType = 7u8;
pub const ELEMENT_TYPE_U4: CorElementType = 9u8;
pub const ELEMENT_TYPE_U8: CorElementType = 11u8;
pub const ELEMENT_TYPE_VALUETYPE: CorElementType = 17u8;
pub const ELEMENT_TYPE_VAR: CorElementType = 19u8;
pub const ELEMENT_TYPE_VOID: CorElementType = 1u8;
pub const FORWARD_INTEROP_STUB_METHOD_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ManagedToNativeComInteropStubAttribute");
pub const FORWARD_INTEROP_STUB_METHOD_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ManagedToNativeComInteropStubAttribute");
pub const FRAMEWORK_REGISTRY_KEY: windows_core::PCSTR = windows_core::s!("Software\\Microsoft\\.NETFramework");
pub const FRAMEWORK_REGISTRY_KEY_W: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\.NETFramework");
pub const FRIEND_ACCESS_ALLOWED_ATTRIBUTE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.FriendAccessAllowedAttribute");
pub const FRIEND_ACCESS_ALLOWED_ATTRIBUTE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.FriendAccessAllowedAttribute");
pub const FRIEND_ASSEMBLY_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.InternalsVisibleToAttribute");
pub const FRIEND_ASSEMBLY_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.InternalsVisibleToAttribute");
pub const GUID_DispIdOverride: windows_core::GUID = windows_core::GUID::from_u128(0xcd2bc5c9_f452_4326_b714_f9c539d4da58);
pub const GUID_ExportedFromComPlus: windows_core::GUID = windows_core::GUID::from_u128(0x90883f05_3d28_11d2_8f17_00a0c9a6186d);
pub const GUID_ForceIEnumerable: windows_core::GUID = windows_core::GUID::from_u128(0xb64784eb_d8d4_4d9b_9acd_0e30806426f7);
pub const GUID_Function2Getter: windows_core::GUID = windows_core::GUID::from_u128(0x54fc8f55_38de_4703_9c4e_250351302b1c);
pub const GUID_ManagedName: windows_core::GUID = windows_core::GUID::from_u128(0x0f21f359_ab84_41e8_9a78_36d110e6d2f9);
pub const GUID_PropGetCA: windows_core::GUID = windows_core::GUID::from_u128(0x2941ff83_88d8_4f73_b6a9_bdf8712d000d);
pub const GUID_PropPutCA: windows_core::GUID = windows_core::GUID::from_u128(0x29533527_3683_4364_abc0_db1add822fa2);
pub const IMAGE_CEE_CS_BYVALUE: CorArgType = 10i32;
pub const IMAGE_CEE_CS_CALLCONV_C: CorUnmanagedCallingConvention = 1i32;
pub const IMAGE_CEE_CS_CALLCONV_DEFAULT: CorCallingConvention = 0i32;
pub const IMAGE_CEE_CS_CALLCONV_EXPLICITTHIS: CorCallingConvention = 64i32;
pub const IMAGE_CEE_CS_CALLCONV_FASTCALL: CorUnmanagedCallingConvention = 4i32;
pub const IMAGE_CEE_CS_CALLCONV_FIELD: CorCallingConvention = 6i32;
pub const IMAGE_CEE_CS_CALLCONV_GENERIC: CorCallingConvention = 16i32;
pub const IMAGE_CEE_CS_CALLCONV_GENERICINST: CorCallingConvention = 10i32;
pub const IMAGE_CEE_CS_CALLCONV_HASTHIS: CorCallingConvention = 32i32;
pub const IMAGE_CEE_CS_CALLCONV_LOCAL_SIG: CorCallingConvention = 7i32;
pub const IMAGE_CEE_CS_CALLCONV_MASK: CorCallingConvention = 15i32;
pub const IMAGE_CEE_CS_CALLCONV_MAX: CorCallingConvention = 12i32;
pub const IMAGE_CEE_CS_CALLCONV_NATIVEVARARG: CorCallingConvention = 11i32;
pub const IMAGE_CEE_CS_CALLCONV_PROPERTY: CorCallingConvention = 8i32;
pub const IMAGE_CEE_CS_CALLCONV_STDCALL: CorUnmanagedCallingConvention = 2i32;
pub const IMAGE_CEE_CS_CALLCONV_THISCALL: CorUnmanagedCallingConvention = 3i32;
pub const IMAGE_CEE_CS_CALLCONV_UNMGD: CorCallingConvention = 9i32;
pub const IMAGE_CEE_CS_CALLCONV_VARARG: CorCallingConvention = 5i32;
pub const IMAGE_CEE_CS_END: CorArgType = 0i32;
pub const IMAGE_CEE_CS_I4: CorArgType = 2i32;
pub const IMAGE_CEE_CS_I8: CorArgType = 3i32;
pub const IMAGE_CEE_CS_OBJECT: CorArgType = 7i32;
pub const IMAGE_CEE_CS_PTR: CorArgType = 6i32;
pub const IMAGE_CEE_CS_R4: CorArgType = 4i32;
pub const IMAGE_CEE_CS_R8: CorArgType = 5i32;
pub const IMAGE_CEE_CS_STRUCT32: CorArgType = 9i32;
pub const IMAGE_CEE_CS_STRUCT4: CorArgType = 8i32;
pub const IMAGE_CEE_CS_VOID: CorArgType = 1i32;
pub const IMAGE_CEE_UNMANAGED_CALLCONV_C: CorUnmanagedCallingConvention = 1i32;
pub const IMAGE_CEE_UNMANAGED_CALLCONV_FASTCALL: CorUnmanagedCallingConvention = 4i32;
pub const IMAGE_CEE_UNMANAGED_CALLCONV_STDCALL: CorUnmanagedCallingConvention = 2i32;
pub const IMAGE_CEE_UNMANAGED_CALLCONV_THISCALL: CorUnmanagedCallingConvention = 3i32;
pub const IMAGE_DIRECTORY_ENTRY_COMHEADER: ReplacesGeneralNumericDefines = 14i32;
pub const INTEROP_AUTOPROXY_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.AutomationProxyAttribute");
pub const INTEROP_AUTOPROXY_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.AutomationProxyAttribute");
pub const INTEROP_BESTFITMAPPING_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.BestFitMappingAttribute");
pub const INTEROP_BESTFITMAPPING_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.BestFitMappingAttribute");
pub const INTEROP_CLASSINTERFACE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ClassInterfaceAttribute");
pub const INTEROP_CLASSINTERFACE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ClassInterfaceAttribute");
pub const INTEROP_COCLASS_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.CoClassAttribute");
pub const INTEROP_COCLASS_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.CoClassAttribute");
pub const INTEROP_COMALIASNAME_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComAliasNameAttribute");
pub const INTEROP_COMALIASNAME_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComAliasNameAttribute");
pub const INTEROP_COMCOMPATIBLEVERSION_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComCompatibleVersionAttribute");
pub const INTEROP_COMCOMPATIBLEVERSION_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComCompatibleVersionAttribute");
pub const INTEROP_COMCONVERSIONLOSS_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComConversionLossAttribute");
pub const INTEROP_COMCONVERSIONLOSS_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComConversionLossAttribute");
pub const INTEROP_COMDEFAULTINTERFACE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComDefaultInterfaceAttribute");
pub const INTEROP_COMDEFAULTINTERFACE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComDefaultInterfaceAttribute");
pub const INTEROP_COMEMULATE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComEmulateAttribute");
pub const INTEROP_COMEMULATE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComEmulateAttribute");
pub const INTEROP_COMEVENTINTERFACE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComEventInterfaceAttribute");
pub const INTEROP_COMEVENTINTERFACE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComEventInterfaceAttribute");
pub const INTEROP_COMIMPORT_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComImportAttribute");
pub const INTEROP_COMIMPORT_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComImportAttribute");
pub const INTEROP_COMREGISTERFUNCTION_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComRegisterFunctionAttribute");
pub const INTEROP_COMREGISTERFUNCTION_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComRegisterFunctionAttribute");
pub const INTEROP_COMSOURCEINTERFACES_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComSourceInterfacesAttribute");
pub const INTEROP_COMSOURCEINTERFACES_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComSourceInterfacesAttribute");
pub const INTEROP_COMSUBSTITUTABLEINTERFACE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComSubstitutableInterfaceAttribute");
pub const INTEROP_COMSUBSTITUTABLEINTERFACE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComSubstitutableInterfaceAttribute");
pub const INTEROP_COMUNREGISTERFUNCTION_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComUnregisterFunctionAttribute");
pub const INTEROP_COMUNREGISTERFUNCTION_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComUnregisterFunctionAttribute");
pub const INTEROP_COMVISIBLE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ComVisibleAttribute");
pub const INTEROP_COMVISIBLE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ComVisibleAttribute");
pub const INTEROP_DATETIMEVALUE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.DateTimeConstantAttribute");
pub const INTEROP_DATETIMEVALUE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.DateTimeConstantAttribute");
pub const INTEROP_DECIMALVALUE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.DecimalConstantAttribute");
pub const INTEROP_DECIMALVALUE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.DecimalConstantAttribute");
pub const INTEROP_DEFAULTMEMBER_TYPE: windows_core::PCSTR = windows_core::s!("System.Reflection.DefaultMemberAttribute");
pub const INTEROP_DEFAULTMEMBER_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Reflection.DefaultMemberAttribute");
pub const INTEROP_DISPID_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.DispIdAttribute");
pub const INTEROP_DISPID_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.DispIdAttribute");
pub const INTEROP_GUID_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.GuidAttribute");
pub const INTEROP_GUID_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.GuidAttribute");
pub const INTEROP_IDISPATCHIMPL_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.IDispatchImplAttribute");
pub const INTEROP_IDISPATCHIMPL_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.IDispatchImplAttribute");
pub const INTEROP_IDISPATCHVALUE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.IDispatchConstantAttribute");
pub const INTEROP_IDISPATCHVALUE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.IDispatchConstantAttribute");
pub const INTEROP_IMPORTEDFROMTYPELIB_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.ImportedFromTypeLibAttribute");
pub const INTEROP_IMPORTEDFROMTYPELIB_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.ImportedFromTypeLibAttribute");
pub const INTEROP_INTERFACETYPE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.InterfaceTypeAttribute");
pub const INTEROP_INTERFACETYPE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.InterfaceTypeAttribute");
pub const INTEROP_IN_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.InAttribute");
pub const INTEROP_IN_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.InAttribute");
pub const INTEROP_IUNKNOWNVALUE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.IUnknownConstantAttribute");
pub const INTEROP_IUNKNOWNVALUE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.IUnknownConstantAttribute");
pub const INTEROP_LCIDCONVERSION_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.LCIDConversionAttribute");
pub const INTEROP_LCIDCONVERSION_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.LCIDConversionAttribute");
pub const INTEROP_MARSHALAS_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.MarshalAsAttribute");
pub const INTEROP_MARSHALAS_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.MarshalAsAttribute");
pub const INTEROP_OUT_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.OutAttribute");
pub const INTEROP_OUT_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.OutAttribute");
pub const INTEROP_PARAMARRAY_TYPE: windows_core::PCSTR = windows_core::s!("System.ParamArrayAttribute");
pub const INTEROP_PARAMARRAY_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.ParamArrayAttribute");
pub const INTEROP_PRESERVESIG_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.PreserveSigAttribure");
pub const INTEROP_PRESERVESIG_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.PreserveSigAttribure");
pub const INTEROP_PRIMARYINTEROPASSEMBLY_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.PrimaryInteropAssemblyAttribute");
pub const INTEROP_PRIMARYINTEROPASSEMBLY_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.PrimaryInteropAssemblyAttribute");
pub const INTEROP_SERIALIZABLE_TYPE: windows_core::PCSTR = windows_core::s!("System.SerializableAttribute");
pub const INTEROP_SERIALIZABLE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.SerializableAttribute");
pub const INTEROP_SETWIN32CONTEXTINIDISPATCHATTRIBUTE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.SetWin32ContextInIDispatchAttribute");
pub const INTEROP_SETWIN32CONTEXTINIDISPATCHATTRIBUTE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.SetWin32ContextInIDispatchAttribute");
pub const INTEROP_TYPELIBFUNC_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.TypeLibFuncAttribute");
pub const INTEROP_TYPELIBFUNC_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.TypeLibFuncAttribute");
pub const INTEROP_TYPELIBIMPORTCLASS_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.TypeLibImportClassAttribute");
pub const INTEROP_TYPELIBIMPORTCLASS_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.TypeLibImportClassAttribute");
pub const INTEROP_TYPELIBTYPE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.TypeLibTypeAttribute");
pub const INTEROP_TYPELIBTYPE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.TypeLibTypeAttribute");
pub const INTEROP_TYPELIBVAR_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.TypeLibVarAttribute");
pub const INTEROP_TYPELIBVAR_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.TypeLibVarAttribute");
pub const INTEROP_TYPELIBVERSION_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.InteropServices.TypeLibVersionAttribute");
pub const INTEROP_TYPELIBVERSION_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.InteropServices.TypeLibVersionAttribute");
pub const INVALID_CONNECTION_ID: u32 = 0u32;
pub const INVALID_TASK_ID: u32 = 0u32;
pub const LIBID_ComPlusRuntime: windows_core::GUID = windows_core::GUID::from_u128(0xbed7f4ea_1a96_11d2_8f08_00a0c9a6186d);
pub const LoadAlways: LoadHintEnum = 1i32;
pub const LoadDefault: LoadHintEnum = 0i32;
pub const LoadNever: LoadHintEnum = 3i32;
pub const LoadSometimes: LoadHintEnum = 2i32;
pub const MAIN_CLR_MODULE_NAME_A: windows_core::PCSTR = windows_core::s!("coreclr");
pub const MAIN_CLR_MODULE_NAME_W: windows_core::PCWSTR = windows_core::w!("coreclr");
pub const MAX_CONNECTION_NAME: u32 = 260u32;
pub const MDAssembly: CorLinkerOptions = 0i32;
pub const MDDupAll: CorCheckDuplicatesFor = -1i32;
pub const MDDupAssembly: CorCheckDuplicatesFor = 268435456i32;
pub const MDDupAssemblyRef: CorCheckDuplicatesFor = 32768i32;
pub const MDDupCustomAttribute: CorCheckDuplicatesFor = 32i32;
pub const MDDupDefault: CorCheckDuplicatesFor = 1058840i32;
pub const MDDupENC: CorCheckDuplicatesFor = -1i32;
pub const MDDupEvent: CorCheckDuplicatesFor = 512i32;
pub const MDDupExportedType: CorCheckDuplicatesFor = 131072i32;
pub const MDDupFieldDef: CorCheckDuplicatesFor = 1024i32;
pub const MDDupFile: CorCheckDuplicatesFor = 65536i32;
pub const MDDupGenericParam: CorCheckDuplicatesFor = 524288i32;
pub const MDDupGenericParamConstraint: CorCheckDuplicatesFor = 2097152i32;
pub const MDDupImplMap: CorCheckDuplicatesFor = 16384i32;
pub const MDDupInterfaceImpl: CorCheckDuplicatesFor = 2i32;
pub const MDDupManifestResource: CorCheckDuplicatesFor = 262144i32;
pub const MDDupMemberRef: CorCheckDuplicatesFor = 16i32;
pub const MDDupMethodDef: CorCheckDuplicatesFor = 4i32;
pub const MDDupMethodSpec: CorCheckDuplicatesFor = 1048576i32;
pub const MDDupModuleRef: CorCheckDuplicatesFor = 4096i32;
pub const MDDupParamDef: CorCheckDuplicatesFor = 64i32;
pub const MDDupPermission: CorCheckDuplicatesFor = 128i32;
pub const MDDupProperty: CorCheckDuplicatesFor = 256i32;
pub const MDDupSignature: CorCheckDuplicatesFor = 2048i32;
pub const MDDupTypeDef: CorCheckDuplicatesFor = 1i32;
pub const MDDupTypeRef: CorCheckDuplicatesFor = 8i32;
pub const MDDupTypeSpec: CorCheckDuplicatesFor = 8192i32;
pub const MDErrorOutOfOrderAll: CorErrorIfEmitOutOfOrder = -1i32;
pub const MDErrorOutOfOrderDefault: CorErrorIfEmitOutOfOrder = 0i32;
pub const MDErrorOutOfOrderNone: CorErrorIfEmitOutOfOrder = 0i32;
pub const MDEventOutOfOrder: CorErrorIfEmitOutOfOrder = 16i32;
pub const MDFieldOutOfOrder: CorErrorIfEmitOutOfOrder = 2i32;
pub const MDImportOptionAll: CorImportOptions = -1i32;
pub const MDImportOptionAllCustomAttributes: CorImportOptions = 32i32;
pub const MDImportOptionAllEvents: CorImportOptions = 16i32;
pub const MDImportOptionAllExportedTypes: CorImportOptions = 64i32;
pub const MDImportOptionAllFieldDefs: CorImportOptions = 4i32;
pub const MDImportOptionAllMethodDefs: CorImportOptions = 2i32;
pub const MDImportOptionAllProperties: CorImportOptions = 8i32;
pub const MDImportOptionAllTypeDefs: CorImportOptions = 1i32;
pub const MDImportOptionDefault: CorImportOptions = 0i32;
pub const MDMemberRefToDef: CorRefToDefCheck = 2i32;
pub const MDMethodOutOfOrder: CorErrorIfEmitOutOfOrder = 1i32;
pub const MDNetModule: CorLinkerOptions = 1i32;
pub const MDNoDupChecks: CorCheckDuplicatesFor = 0i32;
pub const MDNotifyAll: CorNotificationForTokenMovement = -1i32;
pub const MDNotifyAssemblyRef: CorNotificationForTokenMovement = 16777216i32;
pub const MDNotifyCustomAttribute: CorNotificationForTokenMovement = 2048i32;
pub const MDNotifyDefault: CorNotificationForTokenMovement = 15i32;
pub const MDNotifyEvent: CorNotificationForTokenMovement = 256i32;
pub const MDNotifyExportedType: CorNotificationForTokenMovement = 67108864i32;
pub const MDNotifyFieldDef: CorNotificationForTokenMovement = 4i32;
pub const MDNotifyFile: CorNotificationForTokenMovement = 33554432i32;
pub const MDNotifyInterfaceImpl: CorNotificationForTokenMovement = 64i32;
pub const MDNotifyMemberRef: CorNotificationForTokenMovement = 2i32;
pub const MDNotifyMethodDef: CorNotificationForTokenMovement = 1i32;
pub const MDNotifyModuleRef: CorNotificationForTokenMovement = 16384i32;
pub const MDNotifyNameSpace: CorNotificationForTokenMovement = 32768i32;
pub const MDNotifyNone: CorNotificationForTokenMovement = 0i32;
pub const MDNotifyParamDef: CorNotificationForTokenMovement = 32i32;
pub const MDNotifyPermission: CorNotificationForTokenMovement = 8192i32;
pub const MDNotifyProperty: CorNotificationForTokenMovement = 128i32;
pub const MDNotifyResource: CorNotificationForTokenMovement = 134217728i32;
pub const MDNotifySecurityValue: CorNotificationForTokenMovement = 4096i32;
pub const MDNotifySignature: CorNotificationForTokenMovement = 512i32;
pub const MDNotifyTypeDef: CorNotificationForTokenMovement = 16i32;
pub const MDNotifyTypeRef: CorNotificationForTokenMovement = 8i32;
pub const MDNotifyTypeSpec: CorNotificationForTokenMovement = 1024i32;
pub const MDParamOutOfOrder: CorErrorIfEmitOutOfOrder = 4i32;
pub const MDPreserveLocalMemberRef: CorLocalRefPreservation = 2i32;
pub const MDPreserveLocalRefsNone: CorLocalRefPreservation = 0i32;
pub const MDPreserveLocalTypeRef: CorLocalRefPreservation = 1i32;
pub const MDPropertyOutOfOrder: CorErrorIfEmitOutOfOrder = 8i32;
pub const MDRefToDefAll: CorRefToDefCheck = -1i32;
pub const MDRefToDefDefault: CorRefToDefCheck = 3i32;
pub const MDRefToDefNone: CorRefToDefCheck = 0i32;
pub const MDSetENCOff: CorSetENC = 2i32;
pub const MDSetENCOn: CorSetENC = 1i32;
pub const MDThreadSafetyDefault: CorThreadSafetyOptions = 0i32;
pub const MDThreadSafetyOff: CorThreadSafetyOptions = 0i32;
pub const MDThreadSafetyOn: CorThreadSafetyOptions = 1i32;
pub const MDTypeRefToDef: CorRefToDefCheck = 1i32;
pub const MDUpdateDelta: CorSetENC = 5i32;
pub const MDUpdateENC: CorSetENC = 1i32;
pub const MDUpdateExtension: CorSetENC = 3i32;
pub const MDUpdateFull: CorSetENC = 2i32;
pub const MDUpdateIncremental: CorSetENC = 4i32;
pub const MDUpdateMask: CorSetENC = 7i32;
pub const MSCOREE_SHIM_A: windows_core::PCSTR = windows_core::s!("mscoree.dll");
pub const MSCOREE_SHIM_W: windows_core::PCWSTR = windows_core::w!("mscoree.dll");
pub const MergeExportedTypes: MergeFlags = 8i32;
pub const MergeFlagsNone: MergeFlags = 0i32;
pub const MergeManifest: MergeFlags = 1i32;
pub const MetaDataCheckDuplicatesFor: windows_core::GUID = windows_core::GUID::from_u128(0x30fe7be8_d7d9_11d2_9f80_00c04f79a0a3);
pub const MetaDataErrorIfEmitOutOfOrder: windows_core::GUID = windows_core::GUID::from_u128(0x1547872d_dc03_11d2_9420_0000f8083460);
pub const MetaDataGenerateTCEAdapters: windows_core::GUID = windows_core::GUID::from_u128(0xdcc9de90_4151_11d3_88d6_00902754c43a);
pub const MetaDataImportOption: windows_core::GUID = windows_core::GUID::from_u128(0x79700f36_4aac_11d3_84c3_009027868cb1);
pub const MetaDataLinkerOptions: windows_core::GUID = windows_core::GUID::from_u128(0x47e099b6_ae7c_4797_8317_b48aa645b8f9);
pub const MetaDataMergerOptions: windows_core::GUID = windows_core::GUID::from_u128(0x132d3a6e_b35d_464e_951a_42efb9fb6601);
pub const MetaDataNotificationForTokenMovement: windows_core::GUID = windows_core::GUID::from_u128(0xe5d71a4c_d7da_11d2_9f80_00c04f79a0a3);
pub const MetaDataPreserveLocalRefs: windows_core::GUID = windows_core::GUID::from_u128(0xa55c0354_e91b_468b_8648_7cc31035d533);
pub const MetaDataRefToDefCheck: windows_core::GUID = windows_core::GUID::from_u128(0xde3856f8_d7d9_11d2_9f80_00c04f79a0a3);
pub const MetaDataRuntimeVersion: windows_core::GUID = windows_core::GUID::from_u128(0x47e099b7_ae7c_4797_8317_b48aa645b8f9);
pub const MetaDataSetUpdate: windows_core::GUID = windows_core::GUID::from_u128(0x2eee315c_d7db_11d2_9f80_00c04f79a0a3);
pub const MetaDataThreadSafetyOptions: windows_core::GUID = windows_core::GUID::from_u128(0xf7559806_f266_42ea_8c63_0adb45e8b234);
pub const MetaDataTypeLibImportNamespace: windows_core::GUID = windows_core::GUID::from_u128(0xf17ff889_5a63_11d3_9ff2_00c04ff7431a);
pub const NATIVE_TYPE_ANSIBSTR: CorNativeType = 35i32;
pub const NATIVE_TYPE_ARRAY: CorNativeType = 42i32;
pub const NATIVE_TYPE_ASANY: CorNativeType = 40i32;
pub const NATIVE_TYPE_BOOLEAN: CorNativeType = 2i32;
pub const NATIVE_TYPE_BSTR: CorNativeType = 19i32;
pub const NATIVE_TYPE_BYVALSTR: CorNativeType = 34i32;
pub const NATIVE_TYPE_CURRENCY: CorNativeType = 15i32;
pub const NATIVE_TYPE_CUSTOMMARSHALER: CorNativeType = 44i32;
pub const NATIVE_TYPE_DATE: CorNativeType = 18i32;
pub const NATIVE_TYPE_DECIMAL: CorNativeType = 17i32;
pub const NATIVE_TYPE_END: CorNativeType = 0i32;
pub const NATIVE_TYPE_ERROR: CorNativeType = 45i32;
pub const NATIVE_TYPE_FIXEDARRAY: CorNativeType = 30i32;
pub const NATIVE_TYPE_FIXEDSYSSTRING: CorNativeType = 23i32;
pub const NATIVE_TYPE_FUNC: CorNativeType = 38i32;
pub const NATIVE_TYPE_HSTRING: CorNativeType = 47i32;
pub const NATIVE_TYPE_I1: CorNativeType = 3i32;
pub const NATIVE_TYPE_I2: CorNativeType = 5i32;
pub const NATIVE_TYPE_I4: CorNativeType = 7i32;
pub const NATIVE_TYPE_I8: CorNativeType = 9i32;
pub const NATIVE_TYPE_IDISPATCH: CorNativeType = 26i32;
pub const NATIVE_TYPE_IINSPECTABLE: CorNativeType = 46i32;
pub const NATIVE_TYPE_INT: CorNativeType = 31i32;
pub const NATIVE_TYPE_INTF: CorNativeType = 28i32;
pub const NATIVE_TYPE_IUNKNOWN: CorNativeType = 25i32;
pub const NATIVE_TYPE_LPSTR: CorNativeType = 20i32;
pub const NATIVE_TYPE_LPSTRUCT: CorNativeType = 43i32;
pub const NATIVE_TYPE_LPTSTR: CorNativeType = 22i32;
pub const NATIVE_TYPE_LPUTF8STR: CorNativeType = 48i32;
pub const NATIVE_TYPE_LPWSTR: CorNativeType = 21i32;
pub const NATIVE_TYPE_MAX: CorNativeType = 80i32;
pub const NATIVE_TYPE_NESTEDSTRUCT: CorNativeType = 33i32;
pub const NATIVE_TYPE_OBJECTREF: CorNativeType = 24i32;
pub const NATIVE_TYPE_PTR: CorNativeType = 16i32;
pub const NATIVE_TYPE_R4: CorNativeType = 11i32;
pub const NATIVE_TYPE_R8: CorNativeType = 12i32;
pub const NATIVE_TYPE_SAFEARRAY: CorNativeType = 29i32;
pub const NATIVE_TYPE_STRUCT: CorNativeType = 27i32;
pub const NATIVE_TYPE_SYSCHAR: CorNativeType = 13i32;
pub const NATIVE_TYPE_TBSTR: CorNativeType = 36i32;
pub const NATIVE_TYPE_U1: CorNativeType = 4i32;
pub const NATIVE_TYPE_U2: CorNativeType = 6i32;
pub const NATIVE_TYPE_U4: CorNativeType = 8i32;
pub const NATIVE_TYPE_U8: CorNativeType = 10i32;
pub const NATIVE_TYPE_UINT: CorNativeType = 32i32;
pub const NATIVE_TYPE_VARIANT: CorNativeType = 14i32;
pub const NATIVE_TYPE_VARIANTBOOL: CorNativeType = 37i32;
pub const NATIVE_TYPE_VOID: CorNativeType = 1i32;
pub const NGenDefault: NGenHintEnum = 0i32;
pub const NGenEager: NGenHintEnum = 1i32;
pub const NGenLazy: NGenHintEnum = 2i32;
pub const NGenNever: NGenHintEnum = 3i32;
pub const NONVERSIONABLE_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.Versioning.NonVersionableAttribute");
pub const NONVERSIONABLE_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.Versioning.NonVersionableAttribute");
pub const NoDupCheck: MergeFlags = 4i32;
pub const RUNTIMECOMPATIBILITY_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.RuntimeCompatibilityAttribute");
pub const RUNTIMECOMPATIBILITY_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.RuntimeCompatibilityAttribute");
pub const SERIALIZATION_TYPE_BOOLEAN: CorSerializationType = 2i32;
pub const SERIALIZATION_TYPE_CHAR: CorSerializationType = 3i32;
pub const SERIALIZATION_TYPE_ENUM: CorSerializationType = 85i32;
pub const SERIALIZATION_TYPE_FIELD: CorSerializationType = 83i32;
pub const SERIALIZATION_TYPE_I1: CorSerializationType = 4i32;
pub const SERIALIZATION_TYPE_I2: CorSerializationType = 6i32;
pub const SERIALIZATION_TYPE_I4: CorSerializationType = 8i32;
pub const SERIALIZATION_TYPE_I8: CorSerializationType = 10i32;
pub const SERIALIZATION_TYPE_PROPERTY: CorSerializationType = 84i32;
pub const SERIALIZATION_TYPE_R4: CorSerializationType = 12i32;
pub const SERIALIZATION_TYPE_R8: CorSerializationType = 13i32;
pub const SERIALIZATION_TYPE_STRING: CorSerializationType = 14i32;
pub const SERIALIZATION_TYPE_SZARRAY: CorSerializationType = 29i32;
pub const SERIALIZATION_TYPE_TAGGED_OBJECT: CorSerializationType = 81i32;
pub const SERIALIZATION_TYPE_TYPE: CorSerializationType = 80i32;
pub const SERIALIZATION_TYPE_U1: CorSerializationType = 5i32;
pub const SERIALIZATION_TYPE_U2: CorSerializationType = 7i32;
pub const SERIALIZATION_TYPE_U4: CorSerializationType = 9i32;
pub const SERIALIZATION_TYPE_U8: CorSerializationType = 11i32;
pub const SERIALIZATION_TYPE_UNDEFINED: CorSerializationType = 0i32;
pub const SIGN_MASK_FOURBYTE: i32 = -268435456i32;
pub const SIGN_MASK_ONEBYTE: i32 = -64i32;
pub const SIGN_MASK_TWOBYTE: i32 = -8192i32;
pub const SUBJECT_ASSEMBLY_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.CompilerServices.IgnoresAccessChecksToAttribute");
pub const SUBJECT_ASSEMBLY_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.CompilerServices.IgnoresAccessChecksToAttribute");
pub const TARGET_FRAMEWORK_TYPE: windows_core::PCSTR = windows_core::s!("System.Runtime.Versioning.TargetFrameworkAttribute");
pub const TARGET_FRAMEWORK_TYPE_W: windows_core::PCWSTR = windows_core::w!("System.Runtime.Versioning.TargetFrameworkAttribute");
pub const USER_FRAMEWORK_REGISTRY_KEY: windows_core::PCSTR = windows_core::s!("Software\\Microsoft\\.NETFramework64");
pub const USER_FRAMEWORK_REGISTRY_KEY_W: windows_core::PCWSTR = windows_core::w!("Software\\Microsoft\\.NETFramework64");
pub const ValidatorModuleTypeEnc: CorValidatorModuleType = 3i32;
pub const ValidatorModuleTypeIncr: CorValidatorModuleType = 4i32;
pub const ValidatorModuleTypeInvalid: CorValidatorModuleType = 0i32;
pub const ValidatorModuleTypeMax: CorValidatorModuleType = 4i32;
pub const ValidatorModuleTypeMin: CorValidatorModuleType = 1i32;
pub const ValidatorModuleTypeObj: CorValidatorModuleType = 2i32;
pub const ValidatorModuleTypePE: CorValidatorModuleType = 1i32;
pub const afContentType_Default: CorAssemblyFlags = 0i32;
pub const afContentType_Mask: CorAssemblyFlags = 3584i32;
pub const afContentType_WindowsRuntime: CorAssemblyFlags = 512i32;
pub const afDisableJITcompileOptimizer: CorAssemblyFlags = 16384i32;
pub const afEnableJITcompileTracking: CorAssemblyFlags = 32768i32;
pub const afPA_AMD64: CorAssemblyFlags = 64i32;
pub const afPA_ARM: CorAssemblyFlags = 80i32;
pub const afPA_FullMask: CorAssemblyFlags = 240i32;
pub const afPA_IA64: CorAssemblyFlags = 48i32;
pub const afPA_MSIL: CorAssemblyFlags = 16i32;
pub const afPA_Mask: CorAssemblyFlags = 112i32;
pub const afPA_NoPlatform: CorAssemblyFlags = 112i32;
pub const afPA_None: CorAssemblyFlags = 0i32;
pub const afPA_Shift: CorAssemblyFlags = 4i32;
pub const afPA_Specified: CorAssemblyFlags = 128i32;
pub const afPA_x86: CorAssemblyFlags = 32i32;
pub const afPublicKey: CorAssemblyFlags = 1i32;
pub const afRetargetable: CorAssemblyFlags = 256i32;
pub const catAll: CorAttributeTargets = 24575i32;
pub const catAssembly: CorAttributeTargets = 1i32;
pub const catClass: CorAttributeTargets = 4i32;
pub const catClassMembers: CorAttributeTargets = 6140i32;
pub const catConstructor: CorAttributeTargets = 32i32;
pub const catDelegate: CorAttributeTargets = 4096i32;
pub const catEnum: CorAttributeTargets = 16i32;
pub const catEvent: CorAttributeTargets = 512i32;
pub const catField: CorAttributeTargets = 256i32;
pub const catGenericParameter: CorAttributeTargets = 16384i32;
pub const catInterface: CorAttributeTargets = 1024i32;
pub const catMethod: CorAttributeTargets = 64i32;
pub const catModule: CorAttributeTargets = 2i32;
pub const catParameter: CorAttributeTargets = 2048i32;
pub const catProperty: CorAttributeTargets = 128i32;
pub const catStruct: CorAttributeTargets = 8i32;
pub const cssAccurate: CorSaveSize = 0i32;
pub const cssDiscardTransientCAs: CorSaveSize = 2i32;
pub const cssQuick: CorSaveSize = 1i32;
pub const dclActionMask: CorDeclSecurity = 31i32;
pub const dclActionNil: CorDeclSecurity = 0i32;
pub const dclAssert: CorDeclSecurity = 3i32;
pub const dclDemand: CorDeclSecurity = 2i32;
pub const dclDeny: CorDeclSecurity = 4i32;
pub const dclInheritanceCheck: CorDeclSecurity = 7i32;
pub const dclLinktimeCheck: CorDeclSecurity = 6i32;
pub const dclMaximumValue: CorDeclSecurity = 15i32;
pub const dclNonCasDemand: CorDeclSecurity = 13i32;
pub const dclNonCasInheritance: CorDeclSecurity = 15i32;
pub const dclNonCasLinkDemand: CorDeclSecurity = 14i32;
pub const dclPermitOnly: CorDeclSecurity = 5i32;
pub const dclPrejitDenied: CorDeclSecurity = 12i32;
pub const dclPrejitGrant: CorDeclSecurity = 11i32;
pub const dclRequest: CorDeclSecurity = 1i32;
pub const dclRequestMinimum: CorDeclSecurity = 8i32;
pub const dclRequestOptional: CorDeclSecurity = 9i32;
pub const dclRequestRefuse: CorDeclSecurity = 10i32;
pub const evRTSpecialName: CorEventAttr = 1024i32;
pub const evReservedMask: CorEventAttr = 1024i32;
pub const evSpecialName: CorEventAttr = 512i32;
pub const fdAssembly: CorFieldAttr = 3i32;
pub const fdFamANDAssem: CorFieldAttr = 2i32;
pub const fdFamORAssem: CorFieldAttr = 5i32;
pub const fdFamily: CorFieldAttr = 4i32;
pub const fdFieldAccessMask: CorFieldAttr = 7i32;
pub const fdHasDefault: CorFieldAttr = 32768i32;
pub const fdHasFieldMarshal: CorFieldAttr = 4096i32;
pub const fdHasFieldRVA: CorFieldAttr = 256i32;
pub const fdInitOnly: CorFieldAttr = 32i32;
pub const fdLiteral: CorFieldAttr = 64i32;
pub const fdNotSerialized: CorFieldAttr = 128i32;
pub const fdPinvokeImpl: CorFieldAttr = 8192i32;
pub const fdPrivate: CorFieldAttr = 1i32;
pub const fdPrivateScope: CorFieldAttr = 0i32;
pub const fdPublic: CorFieldAttr = 6i32;
pub const fdRTSpecialName: CorFieldAttr = 1024i32;
pub const fdReservedMask: CorFieldAttr = 38144i32;
pub const fdSpecialName: CorFieldAttr = 512i32;
pub const fdStatic: CorFieldAttr = 16i32;
pub const ffContainsMetaData: CorFileFlags = 0i32;
pub const ffContainsNoMetaData: CorFileFlags = 1i32;
pub const fmExecutableImage: CorFileMapping = 1i32;
pub const fmFlat: CorFileMapping = 0i32;
pub const gpContravariant: CorGenericParamAttr = 2i32;
pub const gpCovariant: CorGenericParamAttr = 1i32;
pub const gpDefaultConstructorConstraint: CorGenericParamAttr = 16i32;
pub const gpNoSpecialConstraint: CorGenericParamAttr = 0i32;
pub const gpNonVariant: CorGenericParamAttr = 0i32;
pub const gpNotNullableValueTypeConstraint: CorGenericParamAttr = 8i32;
pub const gpReferenceTypeConstraint: CorGenericParamAttr = 4i32;
pub const gpSpecialConstraintMask: CorGenericParamAttr = 28i32;
pub const gpVarianceMask: CorGenericParamAttr = 3i32;
pub const mdAbstract: CorMethodAttr = 1024i32;
pub const mdAssem: CorMethodAttr = 3i32;
pub const mdCheckAccessOnOverride: CorMethodAttr = 512i32;
pub const mdFamANDAssem: CorMethodAttr = 2i32;
pub const mdFamORAssem: CorMethodAttr = 5i32;
pub const mdFamily: CorMethodAttr = 4i32;
pub const mdFinal: CorMethodAttr = 32i32;
pub const mdHasSecurity: CorMethodAttr = 16384i32;
pub const mdHideBySig: CorMethodAttr = 128i32;
pub const mdMemberAccessMask: CorMethodAttr = 7i32;
pub const mdNewSlot: CorMethodAttr = 256i32;
pub const mdPinvokeImpl: CorMethodAttr = 8192i32;
pub const mdPrivate: CorMethodAttr = 1i32;
pub const mdPrivateScope: CorMethodAttr = 0i32;
pub const mdPublic: CorMethodAttr = 6i32;
pub const mdRTSpecialName: CorMethodAttr = 4096i32;
pub const mdRequireSecObject: CorMethodAttr = 32768i32;
pub const mdReservedMask: CorMethodAttr = 53248i32;
pub const mdReuseSlot: CorMethodAttr = 0i32;
pub const mdSpecialName: CorMethodAttr = 2048i32;
pub const mdStatic: CorMethodAttr = 16i32;
pub const mdUnmanagedExport: CorMethodAttr = 8i32;
pub const mdVirtual: CorMethodAttr = 64i32;
pub const mdVtableLayoutMask: CorMethodAttr = 256i32;
pub const mdtAssembly: CorTokenType = 536870912i32;
pub const mdtAssemblyRef: CorTokenType = 587202560i32;
pub const mdtBaseType: CorTokenType = 1912602624i32;
pub const mdtCustomAttribute: CorTokenType = 201326592i32;
pub const mdtEvent: CorTokenType = 335544320i32;
pub const mdtExportedType: CorTokenType = 654311424i32;
pub const mdtFieldDef: CorTokenType = 67108864i32;
pub const mdtFile: CorTokenType = 637534208i32;
pub const mdtGenericParam: CorTokenType = 704643072i32;
pub const mdtGenericParamConstraint: CorTokenType = 738197504i32;
pub const mdtInterfaceImpl: CorTokenType = 150994944i32;
pub const mdtManifestResource: CorTokenType = 671088640i32;
pub const mdtMemberRef: CorTokenType = 167772160i32;
pub const mdtMethodDef: CorTokenType = 100663296i32;
pub const mdtMethodImpl: CorTokenType = 419430400i32;
pub const mdtMethodSpec: CorTokenType = 721420288i32;
pub const mdtModule: CorTokenType = 0i32;
pub const mdtModuleRef: CorTokenType = 436207616i32;
pub const mdtName: CorTokenType = 1895825408i32;
pub const mdtParamDef: CorTokenType = 134217728i32;
pub const mdtPermission: CorTokenType = 234881024i32;
pub const mdtProperty: CorTokenType = 385875968i32;
pub const mdtSignature: CorTokenType = 285212672i32;
pub const mdtString: CorTokenType = 1879048192i32;
pub const mdtTypeDef: CorTokenType = 33554432i32;
pub const mdtTypeRef: CorTokenType = 16777216i32;
pub const mdtTypeSpec: CorTokenType = 452984832i32;
pub const miAggressiveInlining: CorMethodImpl = 256i32;
pub const miCodeTypeMask: CorMethodImpl = 3i32;
pub const miForwardRef: CorMethodImpl = 16i32;
pub const miIL: CorMethodImpl = 0i32;
pub const miInternalCall: CorMethodImpl = 4096i32;
pub const miManaged: CorMethodImpl = 0i32;
pub const miManagedMask: CorMethodImpl = 4i32;
pub const miMaxMethodImplVal: CorMethodImpl = 65535i32;
pub const miNative: CorMethodImpl = 1i32;
pub const miNoInlining: CorMethodImpl = 8i32;
pub const miNoOptimization: CorMethodImpl = 64i32;
pub const miOPTIL: CorMethodImpl = 2i32;
pub const miPreserveSig: CorMethodImpl = 128i32;
pub const miRuntime: CorMethodImpl = 3i32;
pub const miSecurityMitigations: CorMethodImpl = 1024i32;
pub const miSynchronized: CorMethodImpl = 32i32;
pub const miUnmanaged: CorMethodImpl = 4i32;
pub const miUserMask: CorMethodImpl = 5628i32;
pub const mrPrivate: CorManifestResourceFlags = 2i32;
pub const mrPublic: CorManifestResourceFlags = 1i32;
pub const mrVisibilityMask: CorManifestResourceFlags = 7i32;
pub const msAddOn: CorMethodSemanticsAttr = 8i32;
pub const msFire: CorMethodSemanticsAttr = 32i32;
pub const msGetter: CorMethodSemanticsAttr = 2i32;
pub const msOther: CorMethodSemanticsAttr = 4i32;
pub const msRemoveOn: CorMethodSemanticsAttr = 16i32;
pub const msSetter: CorMethodSemanticsAttr = 1i32;
pub const nlfLastError: CorNativeLinkFlags = 1i32;
pub const nlfMaxValue: CorNativeLinkFlags = 3i32;
pub const nlfNoMangle: CorNativeLinkFlags = 2i32;
pub const nlfNone: CorNativeLinkFlags = 0i32;
pub const nltAnsi: CorNativeLinkType = 2i32;
pub const nltAuto: CorNativeLinkType = 4i32;
pub const nltMaxValue: CorNativeLinkType = 7i32;
pub const nltNone: CorNativeLinkType = 1i32;
pub const nltOle: CorNativeLinkType = 5i32;
pub const nltUnicode: CorNativeLinkType = 3i32;
pub const ntaReserved: NativeTypeArrayFlags = 65534i32;
pub const ntaSizeParamIndexSpecified: NativeTypeArrayFlags = 1i32;
pub const ofCheckIntegrity: CorOpenFlags = 2048i32;
pub const ofCopyMemory: CorOpenFlags = 2i32;
pub const ofNoTransform: CorOpenFlags = 4096i32;
pub const ofNoTypeLib: CorOpenFlags = 128i32;
pub const ofRead: CorOpenFlags = 0i32;
pub const ofReadOnly: CorOpenFlags = 16i32;
pub const ofReadWriteMask: CorOpenFlags = 1i32;
pub const ofReserved: CorOpenFlags = -6336i32;
pub const ofReserved1: CorOpenFlags = 256i32;
pub const ofReserved2: CorOpenFlags = 512i32;
pub const ofReserved3: CorOpenFlags = 1024i32;
pub const ofTakeOwnership: CorOpenFlags = 32i32;
pub const ofWrite: CorOpenFlags = 1i32;
pub const pdHasDefault: CorParamAttr = 4096i32;
pub const pdHasFieldMarshal: CorParamAttr = 8192i32;
pub const pdIn: CorParamAttr = 1i32;
pub const pdOptional: CorParamAttr = 16i32;
pub const pdOut: CorParamAttr = 2i32;
pub const pdReservedMask: CorParamAttr = 61440i32;
pub const pdUnused: CorParamAttr = 53216i32;
pub const pe32BitPreferred: CorPEKind = 16i32;
pub const pe32BitRequired: CorPEKind = 2i32;
pub const pe32Plus: CorPEKind = 4i32;
pub const pe32Unmanaged: CorPEKind = 8i32;
pub const peILonly: CorPEKind = 1i32;
pub const peNot: CorPEKind = 0i32;
pub const pmBestFitDisabled: CorPinvokeMap = 32i32;
pub const pmBestFitEnabled: CorPinvokeMap = 16i32;
pub const pmBestFitMask: CorPinvokeMap = 48i32;
pub const pmBestFitUseAssem: CorPinvokeMap = 0i32;
pub const pmCallConvCdecl: CorPinvokeMap = 512i32;
pub const pmCallConvFastcall: CorPinvokeMap = 1280i32;
pub const pmCallConvMask: CorPinvokeMap = 1792i32;
pub const pmCallConvStdcall: CorPinvokeMap = 768i32;
pub const pmCallConvThiscall: CorPinvokeMap = 1024i32;
pub const pmCallConvWinapi: CorPinvokeMap = 256i32;
pub const pmCharSetAnsi: CorPinvokeMap = 2i32;
pub const pmCharSetAuto: CorPinvokeMap = 6i32;
pub const pmCharSetMask: CorPinvokeMap = 6i32;
pub const pmCharSetNotSpec: CorPinvokeMap = 0i32;
pub const pmCharSetUnicode: CorPinvokeMap = 4i32;
pub const pmMaxValue: CorPinvokeMap = 65535i32;
pub const pmNoMangle: CorPinvokeMap = 1i32;
pub const pmSupportsLastError: CorPinvokeMap = 64i32;
pub const pmThrowOnUnmappableCharDisabled: CorPinvokeMap = 8192i32;
pub const pmThrowOnUnmappableCharEnabled: CorPinvokeMap = 4096i32;
pub const pmThrowOnUnmappableCharMask: CorPinvokeMap = 12288i32;
pub const pmThrowOnUnmappableCharUseAssem: CorPinvokeMap = 0i32;
pub const prHasDefault: CorPropertyAttr = 4096i32;
pub const prRTSpecialName: CorPropertyAttr = 1024i32;
pub const prReservedMask: CorPropertyAttr = 62464i32;
pub const prSpecialName: CorPropertyAttr = 512i32;
pub const prUnused: CorPropertyAttr = 59903i32;
pub const regConfig: CorRegFlags = 2i32;
pub const regHasRefs: CorRegFlags = 4i32;
pub const regNoCopy: CorRegFlags = 1i32;
pub const sdExecute: CeeSectionAttr = 1610612768i64;
pub const sdNone: CeeSectionAttr = 0i64;
pub const sdReadOnly: CeeSectionAttr = 1073741888i64;
pub const sdReadWrite: CeeSectionAttr = 3221225536i64;
pub const srNoBaseReloc: CeeSectionRelocType = 16384i32;
pub const srRelocAbsolute: CeeSectionRelocType = 0i32;
pub const srRelocAbsolutePtr: CeeSectionRelocType = 32768i32;
pub const srRelocAbsoluteTagged: CeeSectionRelocType = 13i32;
pub const srRelocCodeRelative: CeeSectionRelocType = 8i32;
pub const srRelocDir64: CeeSectionRelocType = 10i32;
pub const srRelocDir64Ptr: CeeSectionRelocType = 32778i32;
pub const srRelocFilePos: CeeSectionRelocType = 7i32;
pub const srRelocHighAdj: CeeSectionRelocType = 4i32;
pub const srRelocHighLow: CeeSectionRelocType = 3i32;
pub const srRelocHighLowPtr: CeeSectionRelocType = 32771i32;
pub const srRelocIA64Imm64: CeeSectionRelocType = 9i32;
pub const srRelocIA64Imm64Ptr: CeeSectionRelocType = 32777i32;
pub const srRelocIA64PcRel25: CeeSectionRelocType = 11i32;
pub const srRelocIA64PcRel64: CeeSectionRelocType = 12i32;
pub const srRelocMapToken: CeeSectionRelocType = 5i32;
pub const srRelocPtr: CeeSectionRelocType = 32768i32;
pub const srRelocRelative: CeeSectionRelocType = 6i32;
pub const srRelocRelativePtr: CeeSectionRelocType = 32774i32;
pub const srRelocSentinel: CeeSectionRelocType = 14i32;
pub const tdAbstract: CorTypeAttr = 128i32;
pub const tdAnsiClass: CorTypeAttr = 0i32;
pub const tdAutoClass: CorTypeAttr = 131072i32;
pub const tdAutoLayout: CorTypeAttr = 0i32;
pub const tdBeforeFieldInit: CorTypeAttr = 1048576i32;
pub const tdClass: CorTypeAttr = 0i32;
pub const tdClassSemanticsMask: CorTypeAttr = 32i32;
pub const tdCustomFormatClass: CorTypeAttr = 196608i32;
pub const tdCustomFormatMask: CorTypeAttr = 12582912i32;
pub const tdExplicitLayout: CorTypeAttr = 16i32;
pub const tdForwarder: CorTypeAttr = 2097152i32;
pub const tdHasSecurity: CorTypeAttr = 262144i32;
pub const tdImport: CorTypeAttr = 4096i32;
pub const tdInterface: CorTypeAttr = 32i32;
pub const tdLayoutMask: CorTypeAttr = 24i32;
pub const tdNestedAssembly: CorTypeAttr = 5i32;
pub const tdNestedFamANDAssem: CorTypeAttr = 6i32;
pub const tdNestedFamORAssem: CorTypeAttr = 7i32;
pub const tdNestedFamily: CorTypeAttr = 4i32;
pub const tdNestedPrivate: CorTypeAttr = 3i32;
pub const tdNestedPublic: CorTypeAttr = 2i32;
pub const tdNotPublic: CorTypeAttr = 0i32;
pub const tdPublic: CorTypeAttr = 1i32;
pub const tdRTSpecialName: CorTypeAttr = 2048i32;
pub const tdReservedMask: CorTypeAttr = 264192i32;
pub const tdSealed: CorTypeAttr = 256i32;
pub const tdSequentialLayout: CorTypeAttr = 8i32;
pub const tdSerializable: CorTypeAttr = 8192i32;
pub const tdSpecialName: CorTypeAttr = 1024i32;
pub const tdStringFormatMask: CorTypeAttr = 196608i32;
pub const tdUnicodeClass: CorTypeAttr = 65536i32;
pub const tdVisibilityMask: CorTypeAttr = 7i32;
pub const tdWindowsRuntime: CorTypeAttr = 16384i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COINITICOR(pub i32);
impl windows_core::TypeKind for COINITICOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COINITIEE(pub i32);
impl windows_core::TypeKind for COINITIEE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct COUNINITIEE(pub i32);
impl windows_core::TypeKind for COUNINITIEE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CeeSectionAttr(pub i64);
impl windows_core::TypeKind for CeeSectionAttr {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CeeSectionRelocType(pub i32);
impl windows_core::TypeKind for CeeSectionRelocType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CompilationRelaxationsEnum(pub i32);
impl windows_core::TypeKind for CompilationRelaxationsEnum {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorArgType(pub i32);
impl windows_core::TypeKind for CorArgType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorAssemblyFlags(pub i32);
impl windows_core::TypeKind for CorAssemblyFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorAttributeTargets(pub i32);
impl windows_core::TypeKind for CorAttributeTargets {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorCallingConvention(pub i32);
impl windows_core::TypeKind for CorCallingConvention {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorCheckDuplicatesFor(pub i32);
impl windows_core::TypeKind for CorCheckDuplicatesFor {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorDeclSecurity(pub i32);
impl windows_core::TypeKind for CorDeclSecurity {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorElementType(pub u8);
impl windows_core::TypeKind for CorElementType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorErrorIfEmitOutOfOrder(pub i32);
impl windows_core::TypeKind for CorErrorIfEmitOutOfOrder {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorEventAttr(pub i32);
impl windows_core::TypeKind for CorEventAttr {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorExceptionFlag(pub i32);
impl windows_core::TypeKind for CorExceptionFlag {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorFieldAttr(pub i32);
impl windows_core::TypeKind for CorFieldAttr {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorFileFlags(pub i32);
impl windows_core::TypeKind for CorFileFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorFileMapping(pub i32);
impl windows_core::TypeKind for CorFileMapping {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorGenericParamAttr(pub i32);
impl windows_core::TypeKind for CorGenericParamAttr {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorILMethodFlags(pub i32);
impl windows_core::TypeKind for CorILMethodFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorILMethodSect(pub i32);
impl windows_core::TypeKind for CorILMethodSect {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorImportOptions(pub i32);
impl windows_core::TypeKind for CorImportOptions {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorLinkerOptions(pub i32);
impl windows_core::TypeKind for CorLinkerOptions {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorLocalRefPreservation(pub i32);
impl windows_core::TypeKind for CorLocalRefPreservation {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorManifestResourceFlags(pub i32);
impl windows_core::TypeKind for CorManifestResourceFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorMethodAttr(pub i32);
impl windows_core::TypeKind for CorMethodAttr {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorMethodImpl(pub i32);
impl windows_core::TypeKind for CorMethodImpl {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorMethodSemanticsAttr(pub i32);
impl windows_core::TypeKind for CorMethodSemanticsAttr {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorNativeLinkFlags(pub i32);
impl windows_core::TypeKind for CorNativeLinkFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorNativeLinkType(pub i32);
impl windows_core::TypeKind for CorNativeLinkType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorNativeType(pub i32);
impl windows_core::TypeKind for CorNativeType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorNotificationForTokenMovement(pub i32);
impl windows_core::TypeKind for CorNotificationForTokenMovement {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorOpenFlags(pub i32);
impl windows_core::TypeKind for CorOpenFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorPEKind(pub i32);
impl windows_core::TypeKind for CorPEKind {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorParamAttr(pub i32);
impl windows_core::TypeKind for CorParamAttr {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorPinvokeMap(pub i32);
impl windows_core::TypeKind for CorPinvokeMap {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorPropertyAttr(pub i32);
impl windows_core::TypeKind for CorPropertyAttr {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorRefToDefCheck(pub i32);
impl windows_core::TypeKind for CorRefToDefCheck {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorRegFlags(pub i32);
impl windows_core::TypeKind for CorRegFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorSaveSize(pub i32);
impl windows_core::TypeKind for CorSaveSize {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorSerializationType(pub i32);
impl windows_core::TypeKind for CorSerializationType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorSetENC(pub i32);
impl windows_core::TypeKind for CorSetENC {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorThreadSafetyOptions(pub i32);
impl windows_core::TypeKind for CorThreadSafetyOptions {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorTokenType(pub i32);
impl windows_core::TypeKind for CorTokenType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorTypeAttr(pub i32);
impl windows_core::TypeKind for CorTypeAttr {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorUnmanagedCallingConvention(pub i32);
impl windows_core::TypeKind for CorUnmanagedCallingConvention {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CorValidatorModuleType(pub i32);
impl windows_core::TypeKind for CorValidatorModuleType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LoadHintEnum(pub i32);
impl windows_core::TypeKind for LoadHintEnum {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MergeFlags(pub i32);
impl windows_core::TypeKind for MergeFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NGenHintEnum(pub i32);
impl windows_core::TypeKind for NGenHintEnum {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NativeTypeArrayFlags(pub i32);
impl windows_core::TypeKind for NativeTypeArrayFlags {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ReplacesGeneralNumericDefines(pub i32);
impl windows_core::TypeKind for ReplacesGeneralNumericDefines {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ASSEMBLYMETADATA {
    pub usMajorVersion: u16,
    pub usMinorVersion: u16,
    pub usBuildNumber: u16,
    pub usRevisionNumber: u16,
    pub szLocale: windows_core::PWSTR,
    pub cbLocale: u32,
    pub rProcessor: *mut u32,
    pub ulProcessor: u32,
    pub rOS: *mut OSINFO,
    pub ulOS: u32,
}
impl Default for ASSEMBLYMETADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ASSEMBLYMETADATA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COR_FIELD_OFFSET {
    pub ridOfField: u32,
    pub ulOffset: u32,
}
impl Default for COR_FIELD_OFFSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for COR_FIELD_OFFSET {
    type TypeKind = windows_core::CopyType;
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COR_NATIVE_LINK {
    pub m_linkType: u8,
    pub m_flags: u8,
    pub m_entryPoint: u32,
}
impl Default for COR_NATIVE_LINK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for COR_NATIVE_LINK {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COR_SECATTR {
    pub tkCtor: u32,
    pub pCustomAttribute: *const core::ffi::c_void,
    pub cbCustomAttribute: u32,
}
impl Default for COR_SECATTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for COR_SECATTR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CVStruct {
    pub Major: i16,
    pub Minor: i16,
    pub Sub: i16,
    pub Build: i16,
}
impl Default for CVStruct {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CVStruct {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union CeeSectionRelocExtra {
    pub highAdj: u16,
}
impl Default for CeeSectionRelocExtra {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CeeSectionRelocExtra {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union IMAGE_COR_ILMETHOD {
    pub Tiny: IMAGE_COR_ILMETHOD_TINY,
    pub Fat: IMAGE_COR_ILMETHOD_FAT,
}
impl Default for IMAGE_COR_ILMETHOD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_COR_ILMETHOD_FAT {
    pub _bitfield: u32,
    pub CodeSize: u32,
    pub LocalVarSigTok: u32,
}
impl Default for IMAGE_COR_ILMETHOD_FAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_FAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union IMAGE_COR_ILMETHOD_SECT_EH {
    pub Small: IMAGE_COR_ILMETHOD_SECT_EH_SMALL,
    pub Fat: IMAGE_COR_ILMETHOD_SECT_EH_FAT,
}
impl Default for IMAGE_COR_ILMETHOD_SECT_EH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT {
    pub Flags: CorExceptionFlag,
    pub TryOffset: u32,
    pub TryLength: u32,
    pub HandlerOffset: u32,
    pub HandlerLength: u32,
    pub Anonymous: IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT_0,
}
impl Default for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT_0 {
    pub ClassToken: u32,
    pub FilterOffset: u32,
}
impl Default for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub Anonymous: IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    pub ClassToken: u32,
    pub FilterOffset: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    pub _bitfield1: i32,
    pub _bitfield2: u32,
    pub Anonymous: IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0,
}
#[cfg(target_arch = "x86")]
impl Default for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    pub ClassToken: u32,
    pub FilterOffset: u32,
}
#[cfg(target_arch = "x86")]
impl Default for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_COR_ILMETHOD_SECT_EH_FAT {
    pub SectFat: IMAGE_COR_ILMETHOD_SECT_FAT,
    pub Clauses: [IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_FAT; 1],
}
impl Default for IMAGE_COR_ILMETHOD_SECT_EH_FAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_FAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_COR_ILMETHOD_SECT_EH_SMALL {
    pub SectSmall: IMAGE_COR_ILMETHOD_SECT_SMALL,
    pub Reserved: u16,
    pub Clauses: [IMAGE_COR_ILMETHOD_SECT_EH_CLAUSE_SMALL; 1],
}
impl Default for IMAGE_COR_ILMETHOD_SECT_EH_SMALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_EH_SMALL {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_COR_ILMETHOD_SECT_FAT {
    pub _bitfield: u32,
}
impl Default for IMAGE_COR_ILMETHOD_SECT_FAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_FAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_COR_ILMETHOD_SECT_SMALL {
    pub Kind: u8,
    pub DataSize: u8,
}
impl Default for IMAGE_COR_ILMETHOD_SECT_SMALL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_SECT_SMALL {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_COR_ILMETHOD_TINY {
    pub Flags_CodeSize: u8,
}
impl Default for IMAGE_COR_ILMETHOD_TINY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for IMAGE_COR_ILMETHOD_TINY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_COR_VTABLEFIXUP {
    pub RVA: u32,
    pub Count: u16,
    pub Type: u16,
}
impl Default for IMAGE_COR_VTABLEFIXUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for IMAGE_COR_VTABLEFIXUP {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OSINFO {
    pub dwOSPlatformId: u32,
    pub dwOSMajorVersion: u32,
    pub dwOSMinorVersion: u32,
}
impl Default for OSINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for OSINFO {
    type TypeKind = windows_core::CopyType;
}
