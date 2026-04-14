//*******************************************************************************
//
// Debugger Data Model
//
// Copyright (c) Microsoft Corporation.  All rights reserved.
//
//*******************************************************************************

#ifndef __DBGMODEL_H__
#define __DBGMODEL_H__

#include <winapifamily.h>

#pragma region Desktop Family or WER Package
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WER)

//
// A number of the model APIs have a convention to return extended error information in the resulting object.
// This means that a series of APIs which return model objects will
//
// - On success
//       Always have a valid model object
//
// - On failure
//       Possibly return back a model object which is an error object which contains extended error information
//       beyond the failing HRESULT.  Such objects are immediately convertable to display strings, etc...
//
#define _COM_Errorptr_ _Outptr_
#define _COM_Errorptr_opt_ _Outptr_opt_
#define _COM_Errorptr_opt_result_maybenull_ _Outptr_opt_result_maybenull_

//
// Location:
//
// Defines the location for an object.  This particular variant of Location is the C-COM access struct.
// Note that a location only has meaning in conjunction with a host context.  It is a location within
// a context.  When performing an operation on the location (reading bytes, writing bytes, etc...),
// a valid host context must be supplied.
//
struct Location
{
    // The HostDefined field has two states that are "Non-Opaque" at the API layer.
    //
    //            0: Indicates that the offset is a pointer into virtual address space of the target.
    //     Non-Zero: The defined value is proprietary to the host.  Clients can propagate and change offset.  They cannot
    //               legally change the value.
    //
    // This can be determined by the IsVirtualAddress() method if this structure is built from C++ code.
    //
    ULONG64 HostDefined;
    ULONG64 Offset;

#ifdef __cplusplus

    // Location():
    //
    // Default constructs an equivalent nullptr.
    //
    Location() :
        HostDefined(0),
        Offset(0)
    {
    }

    // Location():
    //
    // Constructs a location from an offset into the virtual address space of the target.
    //
    Location(ULONG64 virtualAddress) :
        HostDefined(0),
        Offset(virtualAddress)
    {
    }

    Location(const Location& src) :
        HostDefined(src.HostDefined),
        Offset(src.Offset)
    {
    }

    Location& operator=(const Location& src)
    {
        HostDefined = src.HostDefined;
        Offset = src.Offset;
        return *this;
    }

    Location& operator=(ULONG64 virtualAddress)
    {
        HostDefined = 0;
        Offset = virtualAddress;
        return *this;
    }

    bool operator==(const Location& rhs) const
    {
        return (rhs.HostDefined == HostDefined && rhs.Offset == Offset);
    }

    bool operator!=(const Location& rhs) const
    {
        return !(operator==(rhs));
    }

    Location& operator+=(LONG64 offset)
    {
        Offset += offset;
        return *this;
    }

    Location& operator-=(LONG64 offset)
    {
        Offset -= offset;
        return *this;
    }

    Location operator+(LONG64 offset) const
    {
        Location l = *this;
        l += offset;
        return l;
    }

    Location operator-(LONG64 offset) const
    {
        Location l = *this;
        l -= offset;
        return l;
    }

    // GetOffset():
    //
    // Returns the offset of the location.
    //
    ULONG64 GetOffset() const
    {
        return Offset;
    }

    // IsVirtualAddress():
    //
    // Indicates whether the location refers to a virtual address.
    //
    bool IsVirtualAddress() const
    {
        return (HostDefined == 0);
    }

#endif // __cplusplus

};

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

//
// Stub declarations to aid in bridging data model and target composition interfaces for
// any code which does not include DbgServices.h
// 

#ifndef __DBGSERVICES_H__
struct ISvcProcess;
struct ISvcThread;
struct ISvcExecutionUnit;
struct IDebugServiceManager;
struct ISvcModule;
struct ISvcSymbolType;
#endif // __DBGSERVICES_H__

//**************************************************************************
// Public Interfaces:

// {F2BCE54E-4835-4f8a-836E-7981E29904D1}
DEFINE_GUID(IID_IHostDataModelAccess, 0xf2bce54e, 0x4835, 0x4f8a, 0x83, 0x6e, 0x79, 0x81, 0xe2, 0x99, 0x4, 0xd1);

// {0FC7557D-401D-4fca-9365-DA1E9850697C}
DEFINE_GUID(IID_IKeyStore, 0xfc7557d, 0x401d, 0x4fca, 0x93, 0x65, 0xda, 0x1e, 0x98, 0x50, 0x69, 0x7c);

// {E28C7893-3F4B-4b96-BACA-293CDC55F45D}
DEFINE_GUID(IID_IModelObject, 0xe28c7893, 0x3f4b, 0x4b96, 0xba, 0xca, 0x29, 0x3c, 0xdc, 0x55, 0xf4, 0x5d);

// {D61E19F4-AB3D-4344-9F7B-0993F3D58745}
DEFINE_GUID(IID_IModelObject2, 0xd61e19f4, 0xab3d, 0x4344, 0x9f, 0x7b, 0x09, 0x93, 0xf3, 0xd5, 0x87, 0x45);

// {73FE19F4-A110-4500-8ED9-3C28896F508C}
DEFINE_GUID(IID_IDataModelManager, 0x73fe19f4, 0xa110, 0x4500, 0x8e, 0xd9, 0x3c, 0x28, 0x89, 0x6f, 0x50, 0x8c);

// {F412C5EA-2284-4622-A660-A697160D3312}
DEFINE_GUID(IID_IDataModelManager2, 0xf412c5ea, 0x2284, 0x4622, 0xa6, 0x60, 0xa6, 0x97, 0x16, 0xd, 0x33, 0x12);

// {8642DAF8-6EF5-4753-B53F-D83A5CEE8100}
DEFINE_GUID(IID_IDataModelManager3, 0x8642daf8, 0x6ef5, 0x4753, 0xb5, 0x3f, 0xd8, 0x3a, 0x5c, 0xee, 0x81, 0x00);

// {8898AD97-3A2E-421C-953F-035E15426B7C}
DEFINE_GUID(IID_IDataModelManager4, 0x8898ad97, 0x3a2e, 0x421c, 0x95, 0x3f, 0x03, 0x5e, 0x15, 0x42, 0x6b, 0x7c);

// {5253DCF8-5AFF-4c62-B302-56A289E00998}
DEFINE_GUID(IID_IModelKeyReference, 0x5253dcf8, 0x5aff, 0x4c62, 0xb3, 0x2, 0x56, 0xa2, 0x89, 0xe0, 0x9, 0x98);

// {5A0C63D9-0526-42b8-960C-9516A3254C85}
DEFINE_GUID(IID_IModelPropertyAccessor, 0x5a0c63d9, 0x526, 0x42b8, 0x96, 0xc, 0x95, 0x16, 0xa3, 0x25, 0x4c, 0x85);

// {80600C1F-B90B-4896-82AD-1C00207909E8}
DEFINE_GUID(IID_IModelMethod, 0x80600c1f, 0xb90b, 0x4896, 0x82, 0xad, 0x1c, 0x0, 0x20, 0x79, 0x9, 0xe8);

// {345FA92E-5E00-4319-9CAE-971F7601CDCF}
DEFINE_GUID(IID_IKeyEnumerator, 0x345fa92e, 0x5e00, 0x4319, 0x9c, 0xae, 0x97, 0x1f, 0x76, 0x1, 0xcd, 0xcf);

// {E13613F9-3A3C-40b5-8F48-1E5EBFB9B21B}
DEFINE_GUID(IID_IRawEnumerator, 0xe13613f9, 0x3a3c, 0x40b5, 0x8f, 0x48, 0x1e, 0x5e, 0xbf, 0xb9, 0xb2, 0x1b);

// {FCB98D1D-1114-4fbf-B24C-EFFCB5DEF0D3}
DEFINE_GUID(IID_IDataModelConcept, 0xfcb98d1d, 0x1114, 0x4fbf, 0xb2, 0x4c, 0xef, 0xfc, 0xb5, 0xde, 0xf0, 0xd3);

// {47BBFC0B-0B20-4E0C-882B-465D6CCAC97C}
DEFINE_GUID(IID_INamedModelsEnumerator, 0x47bbfc0b, 0x0b20, 0x4e0c, 0x88, 0x2b, 0x46, 0x5d, 0x6c, 0xca, 0xc9, 0x7c);

// {D28E8D70-6C00-4205-940D-501016601EA3}
DEFINE_GUID(IID_IStringDisplayableConcept, 0xd28e8d70, 0x6c00, 0x4205, 0x94, 0xd, 0x50, 0x10, 0x16, 0x60, 0x1e, 0xa3);

// {C7371568-5C78-4A00-A4AB-6EF8823184CB}
DEFINE_GUID(IID_ICodeAddressConcept, 0xc7371568, 0x5c78, 0x4a00, 0xa4, 0xab, 0x6e, 0xf8, 0x82, 0x31,0x84, 0xcb);

// {A4952C59-7144-4c76-873B-6046C0955FFC}
DEFINE_GUID(IID_IObjectWrapperConcept, 0xa4952c59, 0x7144, 0x4c76, 0x87, 0x3b, 0x60, 0x46, 0xc0, 0x95, 0x5f, 0xfc);

// {E4622136-927D-4490-874F-581F3E4E3688}
DEFINE_GUID(IID_IModelIterator, 0xe4622136, 0x927d, 0x4490, 0x87, 0x4f, 0x58, 0x1f, 0x3e, 0x4e, 0x36, 0x88);

// {F5D49D0C-0B02-4301-9C9B-B3A6037628F3}
DEFINE_GUID(IID_IIterableConcept, 0xf5d49d0c, 0xb02, 0x4301, 0x9c, 0x9b, 0xb3, 0xa6, 0x3, 0x76, 0x28, 0xf3);

// {D1FAD99F-3F53-4457-850C-8051DF2D3FB5}
DEFINE_GUID(IID_IIndexableConcept, 0xd1fad99f, 0x3f53, 0x4457, 0x85, 0xc, 0x80, 0x51, 0xdf, 0x2d, 0x3f, 0xb5);

// {9D6C1D7B-A76F-4618-8068-5F76BD9A4E8A}
DEFINE_GUID(IID_IPreferredRuntimeTypeConcept, 0x9d6c1d7b, 0xa76f, 0x4618, 0x80, 0x68, 0x5f, 0x76, 0xbd, 0x9a, 0x4e, 0x8a);

// {B8C74943-6B2C-4eeb-B5C5-35D378A6D99D}
DEFINE_GUID(IID_IDebugHost, 0xb8c74943, 0x6b2c, 0x4eeb, 0xb5, 0xc5, 0x35, 0xd3, 0x78, 0xa6, 0xd9, 0x9d);

// {A68C70D8-5EC0-46e5-B775-3134A48EA2E3}
DEFINE_GUID(IID_IDebugHostContext, 0xa68c70d8, 0x5ec0, 0x46e5, 0xb7, 0x75, 0x31, 0x34, 0xa4, 0x8e, 0xa2, 0xe3);

// {E92274A2-47F4-4538-A196-B83DB25fE403}
DEFINE_GUID(IID_IDebugHostContext2, 0xe92274a2, 0x47f4, 0x4538, 0xa1, 0x96, 0xb8, 0x3d, 0xb2, 0x5f, 0xe4, 0x03);

// {5E67115D-5449-4553-A9E9-CA446578CAB2}
DEFINE_GUID(IID_IDebugHostContextExtension, 0x5e67115d, 0x5449, 0x4553, 0xa9, 0xe9, 0xca, 0x44, 0x65, 0x78, 0xca, 0xb2);

// {35AE8E40-F234-4ef1-B8EA-0DFBC58A2043}
DEFINE_GUID(IID_IDebugHostContextExtensibility, 0x35ae8e40, 0xf234, 0x4ef1, 0xb8, 0xea, 0xd, 0xfb, 0xc5, 0x8a, 0x20, 0x43);

// {EEB8FB43-B44E-4B0F-B871-65F0886FCAF2}
DEFINE_GUID(IID_IDebugHostContextControl, 0xeeb8fb43, 0xb44e, 0x4b0f, 0xb8, 0x71, 0x65, 0xf0, 0x88, 0x6f, 0xca, 0xf2);

// {6301EEE8-85E3-4058-A7C0-D37E0EA65F75}
DEFINE_GUID(IID_IDebugHostContextAlternator, 0x6301eee8, 0x85e3, 0x4058, 0xa7, 0xc0, 0xd3, 0x7e, 0x0e, 0xa6, 0x5f, 0x75);

// {854FD751-C2E1-4eb2-B525-6619CB97A588}
DEFINE_GUID(IID_IDebugHostSymbols, 0x854fd751, 0xc2e1, 0x4eb2, 0xb5, 0x25, 0x66, 0x19, 0xcb, 0x97, 0xa5, 0x88);

// {6BAF1F48-65EE-4ff2-B3AF-10C7F21D38B2}
DEFINE_GUID(IID_IDebugHostSymbols2, 0x6baf1f48, 0x65ee, 0x4ff2, 0xb3, 0xaf, 0x10, 0xc7, 0xf2, 0x1d, 0x38, 0xb2);

// {212149C9-9183-4a3e-B00E-4FD1DC95339B}
DEFINE_GUID(IID_IDebugHostMemory, 0x212149c9, 0x9183, 0x4a3e, 0xb0, 0xe, 0x4f, 0xd1, 0xdc, 0x95, 0x33, 0x9b);

// {EEA033DE-38F6-416b-A251-1D3771001270}
DEFINE_GUID(IID_IDebugHostMemory2, 0xeea033de, 0x38f6, 0x416b, 0xa2, 0x51, 0x1d, 0x37, 0x71, 0x0, 0x12, 0x70);

// {A515ED09-2BF3-4499-BB03-553790079F84}
DEFINE_GUID(IID_IDebugHostMemory3, 0xa515ed09, 0x2bf3, 0x4499, 0xbb, 0x3, 0x55, 0x37, 0x90, 0x7, 0x9f, 0x84);

// {FE6B3658-DA4B-44e3-8A58-6201322280E6}
DEFINE_GUID(IID_IDebugHostMemory4, 0xfe6b3658, 0xda4b, 0x44e3, 0x8a, 0x58, 0x62, 0x1, 0x32, 0x22, 0x80, 0xe6);

// {DF033400-4912-46e9-BA62-6EF2EB4D87D4}
DEFINE_GUID(IID_IDebugHostMemory5, 0xdf033400, 0x4912, 0x46e9, 0xba, 0x62, 0x6e, 0xf2, 0xeb, 0x4d, 0x87, 0xd4);

// {0F819103-87DE-4e96-8277-E05CD441FB22}
DEFINE_GUID(IID_IDebugHostSymbol, 0xf819103, 0x87de, 0x4e96, 0x82, 0x77, 0xe0, 0x5c, 0xd4, 0x41, 0xfb, 0x22);

// {21515B67-6720-4257-8A68-077DC944471C}
DEFINE_GUID(IID_IDebugHostSymbol2, 0x21515b67, 0x6720, 0x4257, 0x8a, 0x68, 0x7, 0x7d, 0xc9, 0x44, 0x47, 0x1c);

// {1B3FC1B3-D03D-43e0-8EB0-9AA4BAA21EDB}
DEFINE_GUID(IID_IDebugHostSymbol3, 0x1b3fc1b3, 0xd03d, 0x43e0, 0x8e, 0xb0, 0x9a, 0xa4, 0xba, 0xa2, 0x1e, 0xdb);

// {28D96C86-10A3-4976-B14E-EAEF4790AA1F}
DEFINE_GUID(IID_IDebugHostSymbolEnumerator, 0x28d96c86, 0x10a3, 0x4976, 0xb1, 0x4e, 0xea, 0xef, 0x47, 0x90, 0xaa, 0x1f);

// {D49EECE8-8D12-4ce1-AB73-E5B63DF4F9D3}
DEFINE_GUID(IID_IDebugHostSymbolSubstitutionEnumerator, 0xd49eece8, 0x8d12, 0x4ce1, 0xab, 0x73, 0xe5, 0xb6, 0x3d, 0xf4, 0xf9, 0xd3);

// {C9BA3E18-D070-4378-BBD0-34613B346E1E}
DEFINE_GUID(IID_IDebugHostModule, 0xc9ba3e18, 0xd070, 0x4378, 0xbb, 0xd0, 0x34, 0x61, 0x3b, 0x34, 0x6e, 0x1e);

// {B51887E8-BCD0-4e8f-A8C7-434398B78C37}
DEFINE_GUID(IID_IDebugHostModule2, 0xb51887e8, 0xbcd0, 0x4e8f, 0xa8, 0xc7, 0x43, 0x43, 0x98, 0xb7, 0x8c, 0x37);

// {68576417-9fAB-4C69-8977-3A4D87CF08FD}
DEFINE_GUID(IID_IDebugHostModule3, 0x68576417, 0x9fab, 0x4c69, 0x89, 0x77, 0x3a, 0x4d, 0x87, 0xcf, 0x08, 0xfd);

// {41415136-38A4-474f-8E98-57E2DC64E565}
DEFINE_GUID(IID_IDebugHostModule4, 0x41415136, 0x38a4, 0x474f, 0x8e, 0x98, 0x57, 0xe2, 0xdc, 0x64, 0xe5, 0x65);

// {ED36A63D-AD2B-467e-A0CA-4CA949357625}
DEFINE_GUID(IID_IDebugHostModule5, 0xed36a63d, 0xad2b, 0x467e, 0xa0, 0xca, 0x4c, 0xa9, 0x49, 0x35, 0x76, 0x25);

// {F219B848-63B2-4a43-A6C9-72ABF25A9711}
DEFINE_GUID(IID_IDebugHostType, 0xf219b848, 0x63b2, 0x4a43, 0xa6, 0xc9, 0x72, 0xab, 0xf2, 0x5a, 0x97, 0x11);

// {B28632B9-8506-4676-87CE-8F7E05E59876}
DEFINE_GUID(IID_IDebugHostType2, 0xb28632b9, 0x8506, 0x4676, 0x87, 0xce, 0x8f, 0x7e, 0x5, 0xe5, 0x98, 0x76);

// {8B0409AC-C1BB-433D-887A-ED12C3AF0E7D}
DEFINE_GUID(IID_IDebugHostType3, 0x8b0409ac, 0xc1bb, 0x433d, 0x88, 0x7a, 0xed, 0x12, 0xc3, 0xaf, 0xe, 0x7d);

// {77D3CDC6-BD55-42BF-A4FD-D9AA60E3C1E1}
DEFINE_GUID(IID_IDebugHostType4, 0x77d3cdc6, 0xbd55, 0x42bf, 0xa4, 0xfd, 0xd9, 0xaa, 0x60, 0xe3, 0xc1, 0xe1);

// {DB6716CE-8EE8-4C86-89DB-A658915C87F4}
DEFINE_GUID(IID_IDebugHostType5, 0xdb6716ce, 0x8ee8, 0x4c86, 0x89, 0xdb, 0xa6, 0x58, 0x91, 0x5c, 0x87, 0xf4);

// {08B431ED-F684-4480-8C44-B543AA32CEB0}
DEFINE_GUID(IID_IDebugHostType6, 0x08b431ed, 0xf684, 0x4480, 0x8c, 0x44, 0xb5, 0x43, 0xaa, 0x32, 0xce, 0xb0);

// {F4A035C0-4CA0-4B6D-BFD2-B378A0DBFE4C}
DEFINE_GUID(IID_IDebugHostTaggedUnionRangeEnumerator, 0xf4a035c0, 0x4ca0, 0x4b6d, 0xbf, 0xd2, 0xb3, 0x78, 0xa0, 0xdb, 0xfe, 0x4c);

// {62787EDC-FA76-4690-BD71-5E8C3E2937EC}
DEFINE_GUID(IID_IDebugHostConstant, 0x62787edc, 0xfa76, 0x4690, 0xbd, 0x71, 0x5e, 0x8c, 0x3e, 0x29, 0x37, 0xec);

// {31E53A5A-01EE-4BBB-B899-4B46AE7D595C}
DEFINE_GUID(IID_IDebugHostModuleSignature, 0x31e53a5a, 0x01ee, 0x4bbb, 0xb8, 0x99, 0x4b, 0x46, 0xae, 0x7d, 0x59, 0x5c);

// {3AADC353-2B14-4abb-9893-5E03458E07EE}
DEFINE_GUID(IID_IDebugHostTypeSignature, 0x3aadc353, 0x2b14, 0x4abb, 0x98, 0x93, 0x5e, 0x3, 0x45, 0x8e, 0x7, 0xee);

// {E06F6495-16BC-4cc9-B11D-2A6B23FA72F3}
DEFINE_GUID(IID_IDebugHostField, 0xe06f6495, 0x16bc, 0x4cc9, 0xb1, 0x1d, 0x2a, 0x6b, 0x23, 0xfa, 0x72, 0xf3);

// {99468A0B-EA92-4BD4-9EFE-A266160578CA}
DEFINE_GUID(IID_IDebugHostField2, 0x99468a0b, 0xea92, 0x4bd4, 0x9e, 0xfe, 0xa2, 0x66, 0x16, 0x5, 0x78, 0xca);

// {A3D64993-826C-44fa-897D-926F2FE7AD0B}
DEFINE_GUID(IID_IDebugHostData, 0xa3d64993, 0x826c, 0x44fa, 0x89, 0x7d, 0x92, 0x6f, 0x2f, 0xe7, 0xad, 0xb);

// {B94D57D2-390B-40f7-B5B4-B6DB897D974B}
DEFINE_GUID(IID_IDebugHostBaseClass, 0xb94d57d2, 0x390b, 0x40f7, 0xb5, 0xb4, 0xb6, 0xdb, 0x89, 0x7d, 0x97, 0x4b);

// {435460E2-FD3B-4275-B36C-88EF50188588}
DEFINE_GUID(IID_IDebugHostBaseClass2, 0x435460e2, 0xfd3b, 0x4275, 0xb3, 0x6c, 0x88, 0xef, 0x50, 0x18, 0x85, 0x88);

// {C8FF0F0B-FCE9-467e-8BB3-5D69EF109C00}
DEFINE_GUID(IID_IDebugHostErrorSink, 0xc8ff0f0b, 0xfce9, 0x467e, 0x8b, 0xb3, 0x5d, 0x69, 0xef, 0x10, 0x9c, 0x0);

// {0FEF9A21-577E-4997-AC7B-1C4883241D99}
DEFINE_GUID(IID_IDebugHostEvaluator, 0xfef9a21, 0x577e, 0x4997, 0xac, 0x7b, 0x1c, 0x48, 0x83, 0x24, 0x1d, 0x99);

// {6C597AC9-FB4D-4f6d-9F39-22488539F8F4}
DEFINE_GUID(IID_IDebugHostPublic, 0x6c597ac9, 0xfb4d, 0x4f6d, 0x9f, 0x39, 0x22, 0x48, 0x85, 0x39, 0xf8, 0xf4);

// {4F3E1CE2-86B2-4C7A-9C65-D0A9D0EECF44}
DEFINE_GUID(IID_IDebugHostStatus, 0x4f3e1ce2, 0x86b2, 0x4c7a, 0x9c, 0x65, 0xd0, 0xa9, 0xd0, 0xee, 0xcf, 0x44);

// {4A168D3F-04D0-49c4-8F9A-7B5B3108C6C6}
DEFINE_GUID(IID_IDebugHostStatus2, 0x4a168d3f, 0x4d0, 0x49c4, 0x8f, 0x9a, 0x7b, 0x5b, 0x31, 0x8, 0xc6, 0xc6);

// {3B362B0E-89F0-46c6-A663-DFDC95194AEF}
DEFINE_GUID(IID_IDataModelScriptClient, 0x3b362b0e, 0x89f0, 0x46c6, 0xa6, 0x63, 0xdf, 0xdc, 0x95, 0x19, 0x4a, 0xef);

// {1303DEC4-FA3B-4F1B-9224-B953D16BABB5}
DEFINE_GUID(IID_IDataModelScriptTemplate, 0x1303dec4, 0xfa3b, 0x4f1b, 0x92, 0x24, 0xb9, 0x53, 0xd1, 0x6b, 0xab, 0xb5);

// {7B4D30FC-B14A-49f8-8D87-D9A1480C97F7}
DEFINE_GUID(IID_IDataModelScript, 0x7b4d30fc, 0xb14a, 0x49f8, 0x8d, 0x87, 0xd9, 0xa1, 0x48, 0xc, 0x97, 0xf7);

// {7D90CF81-BEE2-4B91-9D49-8FEC0F7D56D1}
DEFINE_GUID(IID_IDataModelScript2, 0x7d90cf81, 0xbee2, 0x4b91, 0x9d, 0x49, 0x8f, 0xec, 0xf, 0x7d, 0x56, 0xd1);

// {513461E0-4FCA-48ce-8658-32F3E2056F3B}
DEFINE_GUID(IID_IDataModelScriptProvider, 0x513461e0, 0x4fca, 0x48ce, 0x86, 0x58, 0x32, 0xf3, 0xe2, 0x5, 0x6f, 0x3b);

// {6FD11E33-E5AD-410b-8011-68C6BC4BF80D}
DEFINE_GUID(IID_IDataModelScriptManager, 0x6fd11e33, 0xe5ad, 0x410b, 0x80, 0x11, 0x68, 0xc6, 0xbc, 0x4b, 0xf8, 0xd);

// {95BA00E2-704A-4fe2-A8F1-A7E7D8FB0941}
DEFINE_GUID(IID_IDataModelScriptProviderEnumerator, 0x95ba00e2, 0x704a, 0x4fe2, 0xa8, 0xf1, 0xa7, 0xe7, 0xd8, 0xfb, 0x9, 0x41);

// {B70334A4-B92C-4570-93A1-D3EB686649A0}
DEFINE_GUID(IID_IDebugHostScriptHost, 0xb70334a4, 0xb92c, 0x4570, 0x93, 0xa1, 0xd3, 0xeb, 0x68, 0x66, 0x49, 0xa0);

// {014D366A-1F23-4981-9219-B2DB8B402054}
DEFINE_GUID(IID_IDataModelScriptHostContext, 0x14d366a, 0x1f23, 0x4981, 0x92, 0x19, 0xb2, 0xdb, 0x8b, 0x40, 0x20, 0x54);

// {AF352B7B-8292-4c01-B360-2DC3696C65E7}
DEFINE_GUID(IID_IDataModelNameBinder, 0xaf352b7b, 0x8292, 0x4c01, 0xb3, 0x60, 0x2d, 0xc3, 0x69, 0x6c, 0x65, 0xe7);

// {E7983FA1-80A7-498c-988F-518DDC5D4025}
DEFINE_GUID(IID_IDynamicKeyProviderConcept, 0xe7983fa1, 0x80a7, 0x498c, 0x98, 0x8f, 0x51, 0x8d, 0xdc, 0x5d, 0x40, 0x25);

// {95A7F7DD-602E-483f-9D06-A15C0EE13174}
DEFINE_GUID(IID_IDynamicConceptProviderConcept, 0x95a7f7dd, 0x602e, 0x483f, 0x9d, 0x6, 0xa1, 0x5c, 0xe, 0xe1, 0x31, 0x74);

// {69CE6AE2-2268-4e6f-B062-20CE62BFE677}
DEFINE_GUID(IID_IDataModelScriptTemplateEnumerator, 0x69ce6ae2, 0x2268, 0x4e6f, 0xb0, 0x62, 0x20, 0xce, 0x62, 0xbf, 0xe6, 0x77);

// {80E2F7C5-7159-4e92-887E-7E0347E88406}
DEFINE_GUID(IID_IModelKeyReference2, 0x80e2f7c5, 0x7159, 0x4e92, 0x88, 0x7e, 0x7e, 0x3, 0x47, 0xe8, 0x84, 0x6);

// {A117A435-1FB4-4092-A2AB-A929576C1E87}
DEFINE_GUID(IID_IDebugHostEvaluator2, 0xa117a435, 0x1fb4, 0x4092, 0xa2, 0xab, 0xa9, 0x29, 0x57, 0x6c, 0x1e, 0x87);

// {D2419F4A-7E8D-4C15-A499-73902B015ABB}
DEFINE_GUID(IID_IDebugHostEvaluator3, 0xd2419f4a, 0x7e8d, 0x4c15, 0xa4, 0x99, 0x73, 0x90, 0x2b, 0x1, 0x5a, 0xbb);

// {3C2B24E1-11D0-4f86-8AE5-4DF166F73253}
DEFINE_GUID(IID_IDebugHostExtensibility, 0x3c2b24e1, 0x11d0, 0x4f86, 0x8a, 0xe5, 0x4d, 0xf1, 0x66, 0xf7, 0x32, 0x53);

// {91CC55E7-2A22-4494-9710-B729DAB48F71}
DEFINE_GUID(IID_IDebugHostExtensibility2, 0x91cc55e7, 0x2a22, 0x4494, 0x97, 0x10, 0xb7, 0x29, 0xda, 0xb4, 0x8f, 0x71);

// {4BE234DE-D397-4378-BBB4-9055A425D7D1}
DEFINE_GUID(IID_IDebugHostExtensibility3, 0x4be234de, 0xd397, 0x4378, 0xbb, 0xb4, 0x90, 0x55, 0xa4, 0x25, 0xd7, 0xd1);

// {DE8E0945-9750-4471-AB76-A8F79D6EC350}
DEFINE_GUID(IID_IDataModelScriptDebug, 0xde8e0945, 0x9750, 0x4471, 0xab, 0x76, 0xa8, 0xf7, 0x9d, 0x6e, 0xc3, 0x50);

// {53159B6D-D4C4-471b-A863-5B110CA800CA}
DEFINE_GUID(IID_IDataModelScriptDebugClient, 0x53159b6d, 0xd4c4, 0x471b, 0xa8, 0x63, 0x5b, 0x11, 0xc, 0xa8, 0x0, 0xca);

// {051364DD-E449-443e-9762-FE578F4A5473}
DEFINE_GUID(IID_IDataModelScriptDebugStack, 0x51364dd, 0xe449, 0x443e, 0x97, 0x62, 0xfe, 0x57, 0x8f, 0x4a, 0x54, 0x73);

// {DEC6ED5E-6360-4941-AB4C-A26409DE4F82}
DEFINE_GUID(IID_IDataModelScriptDebugStackFrame, 0xdec6ed5e, 0x6360, 0x4941, 0xab, 0x4c, 0xa2, 0x64, 0x9, 0xde, 0x4f, 0x82);

// {0F9FEED7-D045-4ac3-98A8-A98942CF6A35}
DEFINE_GUID(IID_IDataModelScriptDebugVariableSetEnumerator, 0xf9feed7, 0xd045, 0x4ac3, 0x98, 0xa8, 0xa9, 0x89, 0x42, 0xcf, 0x6a, 0x35);

// {6BB27B35-02E6-47cb-90A0-5371244032DE}
DEFINE_GUID(IID_IDataModelScriptDebugBreakpoint, 0x6bb27b35, 0x2e6, 0x47cb, 0x90, 0xa0, 0x53, 0x71, 0x24, 0x40, 0x32, 0xde);

// {39484A75-B4F3-4799-86DA-691AFA57B299}
DEFINE_GUID(IID_IDataModelScriptDebugBreakpointEnumerator, 0x39484a75, 0xb4f3, 0x4799, 0x86, 0xda, 0x69, 0x1a, 0xfa, 0x57, 0xb2, 0x99);

// {CBB10ED3-839E-426c-9243-E23535C1AE1A}
DEFINE_GUID(IID_IDataModelScriptDebug2, 0xcbb10ed3, 0x839e, 0x426c, 0x92, 0x43, 0xe2, 0x35, 0x35, 0xc1, 0xae, 0x1a);

// {A7830646-9F0C-4a31-BA19-503F33E6C8A3}
DEFINE_GUID(IID_IComparableConcept, 0xa7830646, 0x9f0c, 0x4a31, 0xba, 0x19, 0x50, 0x3f, 0x33, 0xe6, 0xc8, 0xa3);

// {C52D5D3D-609D-4d5d-8A82-46B0ACDEC4F4}
DEFINE_GUID(IID_IEquatableConcept, 0xc52d5d3d, 0x609d, 0x4d5d, 0x8a, 0x82, 0x46, 0xb0, 0xac, 0xde, 0xc4, 0xf4);

// {2CD9906F-F1B3-4463-828A-0ADDAFE8BAAE}
DEFINE_GUID(IID_IActionableConcept, 0x2cd9906f, 0xf1b3, 0x4463, 0x82, 0x8a, 0xa, 0xdd, 0xaf, 0xe8, 0xba, 0xae);

// {7FC09C9F-632D-48e8-A97B-2F4F2E5C1161}
DEFINE_GUID(IID_IActionQueryConcept, 0x7fc09c9f, 0x632d, 0x48e8, 0xa9, 0x7b, 0x2f, 0x4f, 0x2e, 0x5c, 0x11, 0x61);

// {3DEC5C44-F63A-4ca6-90F0-FD5C269FDA59}
DEFINE_GUID(IID_IActionEnumerator, 0x3dec5c44, 0xf63a, 0x4ca6, 0x90, 0xf0, 0xfd, 0x5c, 0x26, 0x9f, 0xda, 0x59);

// {1A9409F1-F0E0-4b48-9A4E-5783548FB57A}
DEFINE_GUID(IID_IConstructableConcept, 0x1a9409f1, 0xf0e0, 0x4b48, 0x9a, 0x4e, 0x57, 0x83, 0x54, 0x8f, 0xb5, 0x7a);

// {F798139E-1B2C-4077-8D87-9FA5D044F3EB}
DEFINE_GUID(IID_IDeconstructableConcept, 0xf798139e, 0x1b2c, 0x4077, 0x8d, 0x87, 0x9f, 0xa5, 0xd0, 0x44, 0xf3, 0xeb);

// {A754393C-4FBE-4178-8AD5-FE6079AC048D}
DEFINE_GUID(IID_IDebugHostFunctionIntrospection, 0xa754393c, 0x4fbe, 0x4178, 0x8a, 0xd5, 0xfe, 0x60, 0x79, 0xac, 0x04, 0x8d);

// {8E1CB118-AA83-409a-AAE9-C7FF78911A5F}
DEFINE_GUID(IID_IDebugHostFunctionIntrospection2, 0x8e1cb118, 0xaa83, 0x409a, 0xaa, 0xe9, 0xc7, 0xff, 0x78, 0x91, 0x1a, 0x5f);

// {A24E286B-891A-40fc-8A3A-89B66EDDCE57}
DEFINE_GUID(IID_IDebugHostFunctionIntrospection3, 0xa24e286b, 0x891a, 0x40fc, 0x8a, 0x3a, 0x89, 0xb6, 0x6e, 0xdd, 0xce, 0x57);

// {A61ADC36-1ED5-40fe-A976-6A21CD81E811}
DEFINE_GUID(IID_IDebugHostFunctionLocalDetailsEnumerator, 0xa61adc36, 0x1ed5, 0x40fe, 0xa9, 0x76, 0x6a, 0x21, 0xcd, 0x81, 0xe8, 0x11);

// {89280EA8-B3B9-408c-BE16-32AB28F5C0AC}
DEFINE_GUID(IID_IDebugHostFunctionLocalDetails, 0x89280ea8, 0xb3b9, 0x408c, 0xbe, 0x16, 0x32, 0xab, 0x28, 0xf5, 0xc0, 0xac);

// {199A57B0-1967-4363-B25E-90C7E8A07F22}
DEFINE_GUID(IID_IDebugHostFunctionLocalDetails2, 0x199a57b0, 0x1967, 0x4363, 0xb2, 0x5e, 0x90, 0xc7, 0xe8, 0xa0, 0x7f, 0x22);

// {026C9E81-8B9F-4d32-9606-A394EC62B045}
DEFINE_GUID(IID_IDebugHostFunctionLocalStorageEnumerator, 0x26c9e81, 0x8b9f, 0x4d32, 0x96, 0x6, 0xa3, 0x94, 0xec, 0x62, 0xb0, 0x45);

// {2F2F303B-39BE-4b6d-9BFB-4FAA49DBBD45}
DEFINE_GUID(IID_IDebugHostFunctionLocalStorage, 0x2f2f303b, 0x39be, 0x4b6d, 0x9b, 0xfb, 0x4f, 0xaa, 0x49, 0xdb, 0xbd, 0x45);

// {213B3725-36A2-45A0-9EA4-854D46D85195}
DEFINE_GUID(IID_IDebugHostFunctionLocalStorageDetails, 0x213b3725, 0x36a2, 0x45a0, 0x9e, 0xa4, 0x85, 0x4d, 0x46, 0xd8, 0x51, 0x95);

// {63832802-2DB3-4DE7-B76C-197AC15B5EC6}
DEFINE_GUID(IID_IFilteredNamespacePropertyToken, 0x63832802, 0x2db3, 0x4de7, 0xb7, 0x6c, 0x19, 0x7a, 0xc1, 0x5b, 0x5e, 0xc6);


struct DECLSPEC_UUID("F2BCE54E-4835-4f8a-836E-7981E29904D1") IHostDataModelAccess;

struct DECLSPEC_UUID("0FC7557D-401D-4fca-9365-DA1E9850697C") IKeyStore;
struct DECLSPEC_UUID("E28C7893-3F4B-4b96-BACA-293CDC55F45D") IModelObject;
struct DECLSPEC_UUID("D61E19F4-AB3D-4344-9F7B-0993F3D58745") IModelObject2;
struct DECLSPEC_UUID("73FE19F4-A110-4500-8ED9-3C28896F508C") IDataModelManager;
struct DECLSPEC_UUID("F412C5EA-2284-4622-A660-A697160D3312") IDataModelManager2;
struct DECLSPEC_UUID("8642DAF8-6EF5-4753-B53F-D83A5CEE8100") IDataModelManager3;
struct DECLSPEC_UUID("8898AD97-3A2E-421C-953F-035E15426B7C") IDataModelManager4;
struct DECLSPEC_UUID("5253DCF8-5AFF-4c62-B302-56A289E00998") IModelKeyReference;
struct DECLSPEC_UUID("5A0C63D9-0526-42b8-960C-9516A3254C85") IModelPropertyAccessor;
struct DECLSPEC_UUID("80600C1F-B90B-4896-82AD-1C00207909E8") IModelMethod;
struct DECLSPEC_UUID("345FA92E-5E00-4319-9CAE-971F7601CDCF") IKeyEnumerator;
struct DECLSPEC_UUID("E13613F9-3A3C-40b5-8F48-1E5EBFB9B21B") IRawEnumerator;

struct DECLSPEC_UUID("FCB98D1D-1114-4fbf-B24C-EFFCB5DEF0D3") IDataModelConcept;
struct DECLSPEC_UUID("47BBFC0B-0B20-4E0C-882B-465D6CCAC97C") INamedModelsEnumerator;
struct DECLSPEC_UUID("D28E8D70-6C00-4205-940D-501016601EA3") IStringDisplayableConcept;
struct DECLSPEC_UUID("C7371568-5C78-4A00-A4AB-6EF8823184CB") ICodeAddressConcept;
struct DECLSPEC_UUID("A4952C59-7144-4c76-873B-6046C0955FFC") IObjectWrapperConcept;
struct DECLSPEC_UUID("E4622136-927D-4490-874F-581F3E4E3688") IModelIterator;
struct DECLSPEC_UUID("F5D49D0C-0B02-4301-9C9B-B3A6037628F3") IIterableConcept;
struct DECLSPEC_UUID("D1FAD99F-3F53-4457-850C-8051DF2D3FB5") IIndexableConcept;
struct DECLSPEC_UUID("9D6C1D7B-A76F-4618-8068-5F76BD9A4E8A") IPreferredRuntimeTypeConcept;

struct DECLSPEC_UUID("B8C74943-6B2C-4eeb-B5C5-35D378A6D99D") IDebugHost;
struct DECLSPEC_UUID("A68C70D8-5EC0-46e5-B775-3134A48EA2E3") IDebugHostContext;
struct DECLSPEC_UUID("E92274A2-47F4-4538-A196-B83DB25fE403") IDebugHostContext2;
struct DECLSPEC_UUID("5E67115D-5449-4553-A9E9-CA446578CAB2") IDebugHostContextExtension;
struct DECLSPEC_UUID("35AE8E40-F234-4ef1-B8EA-0DFBC58A2043") IDebugHostContextExtensibility;
struct DECLSPEC_UUID("EEB8FB43-B44E-4B0F-B871-65F0886FCAF2") IDebugHostContextControl;
struct DECLSPEC_UUID("6301EEE8-85E3-4058-A7C0-D37E0EA65F75") IDebugHostContextAlternator;
struct DECLSPEC_UUID("854FD751-C2E1-4eb2-B525-6619CB97A588") IDebugHostSymbols;
struct DECLSPEC_UUID("6BAF1F48-65EE-4ff2-B3AF-10C7F21D38B2") IDebugHostSymbols2;
struct DECLSPEC_UUID("31E53A5A-01EE-4BBB-B899-4B46AE7D595C") IDebugHostModuleSignature;
struct DECLSPEC_UUID("3AADC353-2B14-4abb-9893-5E03458E07EE") IDebugHostTypeSignature;
struct DECLSPEC_UUID("212149C9-9183-4a3e-B00E-4FD1DC95339B") IDebugHostMemory;
struct DECLSPEC_UUID("EEA033DE-38F6-416b-A251-1D3771001270") IDebugHostMemory2;
struct DECLSPEC_UUID("A515ED09-2BF3-4499-BB03-553790079F84") IDebugHostMemory3;
struct DECLSPEC_UUID("FE6B3658-DA4B-44e3-8A58-6201322280E6") IDebugHostMemory4;
struct DECLSPEC_UUID("DF033400-4912-46e9-BA62-6EF2EB4D87D4") IDebugHostMemory5;
struct DECLSPEC_UUID("C8FF0F0B-FCE9-467e-8BB3-5D69EF109C00") IDebugHostErrorSink;
struct DECLSPEC_UUID("0FEF9A21-577E-4997-AC7B-1C4883241D99") IDebugHostEvaluator;

struct DECLSPEC_UUID("28D96C86-10A3-4976-B14E-EAEF4790AA1F") IDebugHostSymbolEnumerator;
struct DECLSPEC_UUID("D49EECE8-8D12-4ce1-AB73-E5B63DF4F9D3") IDebugHostSymbolSubstitutionEnumerator;
struct DECLSPEC_UUID("0F819103-87DE-4e96-8277-E05CD441FB22") IDebugHostSymbol;
struct DECLSPEC_UUID("21515B67-6720-4257-8A68-077DC944471C") IDebugHostSymbol2;
struct DECLSPEC_UUID("1B3FC1B3-D03D-43e0-8EB0-9AA4BAA21EDB") IDebugHostSymbol3;
struct DECLSPEC_UUID("C9BA3E18-D070-4378-BBD0-34613B346E1E") IDebugHostModule;
struct DECLSPEC_UUID("B51887E8-BCD0-4e8f-A8C7-434398B78C37") IDebugHostModule2;
struct DECLSPEC_UUID("68576417-9fAB-4C69-8977-3A4D87CF08FD") IDebugHostModule3;
struct DECLSPEC_UUID("41415136-38A4-474f-8E98-57E2DC64E565") IDebugHostModule4;
struct DECLSPEC_UUID("ED36A63D-AD2B-467e-A0CA-4CA949357625") IDebugHostModule5;
struct DECLSPEC_UUID("3AADC353-2B14-4abb-9893-5E03458E07EE") IDebugHostType;
struct DECLSPEC_UUID("B28632B9-8506-4676-87CE-8F7E05E59876") IDebugHostType2;
struct DECLSPEC_UUID("8B0409AC-C1BB-433D-887A-ED12C3AF0E7D") IDebugHostType3;
struct DECLSPEC_UUID("77D3CDC6-BD55-42BF-A4FD-D9AA60E3C1E1") IDebugHostType4;
struct DECLSPEC_UUID("DB6716CE-8EE8-4C86-89DB-A658915C87F4") IDebugHostType5;
struct DECLSPEC_UUID("08B431ED-F684-4480-8C44-B543AA32CEB0") IDebugHostType6;
struct DECLSPEC_UUID("F4A035C0-4CA0-4B6D-BFD2-B378A0DBFE4C") IDebugHostTaggedUnionRangeEnumerator;
struct DECLSPEC_UUID("62787EDC-FA76-4690-BD71-5E8C3E2937EC") IDebugHostConstant;
struct DECLSPEC_UUID("E06F6495-16BC-4cc9-B11D-2A6B23FA72F3") IDebugHostField;
struct DECLSPEC_UUID("99468A0B-EA92-4BD4-9EFE-A266160578CA") IDebugHostField2;
struct DECLSPEC_UUID("A3D64993-826C-44fa-897D-926F2FE7AD0B") IDebugHostData;
struct DECLSPEC_UUID("B94D57D2-390B-40f7-B5B4-B6DB897D974B") IDebugHostBaseClass;
struct DECLSPEC_UUID("435460E2-FD3B-4275-B36C-88EF50188588") IDebugHostBaseClass2;
struct DECLSPEC_UUID("6C597AC9-FB4D-4f6d-9F39-22488539F8F4") IDebugHostPublic;

struct DECLSPEC_UUID("4F3E1CE2-86B2-4C7A-9C65-D0A9D0EECF44") IDebugHostStatus;
struct DECLSPEC_UUID("4A168D3F-04D0-49c4-8F9A-7B5B3108C6C6") IDebugHostStatus2;
struct DECLSPEC_UUID("3B362B0E-89F0-46c6-A663-DFDC95194AEF") IDataModelScriptClient;
struct DECLSPEC_UUID("1303DEC4-FA3B-4F1B-9224-B953D16BABB5") IDataModelScriptTemplate;
struct DECLSPEC_UUID("7B4D30FC-B14A-49f8-8D87-D9A1480C97F7") IDataModelScript;
struct DECLSPEC_UUID("7D90CF81-BEE2-4B91-9D49-8FEC0F7D56D1") IDataModelScript2;

struct DECLSPEC_UUID("513461E0-4FCA-48ce-8658-32F3E2056F3B") IDataModelScriptProvider;
struct DECLSPEC_UUID("6FD11E33-E5AD-410b-8011-68C6BC4BF80D") IDataModelScriptManager;
struct DECLSPEC_UUID("95BA00E2-704A-4fe2-A8F1-A7E7D8FB0941") IDataModelScriptProviderEnumerator;
struct DECLSPEC_UUID("B70334A4-B92C-4570-93A1-D3EB686649A0") IDebugHostScriptHost;
struct DECLSPEC_UUID("014D366A-1F23-4981-9219-B2DB8B402054") IDataModelScriptHostContext;
struct DECLSPEC_UUID("AF352B7B-8292-4c01-B360-2DC3696C65E7") IDataModelNameBinder;
struct DECLSPEC_UUID("69CE6AE2-2268-4e6f-B062-20CE62BFE677") IDataModelScriptTemplateEnumerator;

struct DECLSPEC_UUID("E7983FA1-80A7-498c-988F-518DDC5D4025") IDynamicKeyProviderConcept;
struct DECLSPEC_UUID("95A7F7DD-602E-483f-9D06-A15C0EE13174") IDynamicConceptProviderConcept;
struct DECLSPEC_UUID("80E2F7C5-7159-4e92-887E-7E0347E88406") IModelKeyReference2;
struct DECLSPEC_UUID("A117A435-1FB4-4092-A2AB-A929576C1E87") IDebugHostEvaluator2;
struct DECLSPEC_UUID("D2419F4A-7E8D-4C15-A499-73902B015ABB") IDebugHostEvaluator3;

struct DECLSPEC_UUID("3C2B24E1-11D0-4f86-8AE5-4DF166F73253") IDebugHostExtensibility;
struct DECLSPEC_UUID("91CC55E7-2A22-4494-9710-B729DAB48F71") IDebugHostExtensibility2;
struct DECLSPEC_UUID("4BE234DE-D397-4378-BBB4-9055A425D7D1") IDebugHostExtensibility3;

struct DECLSPEC_UUID("DE8E0945-9750-4471-AB76-A8F79D6EC350") IDataModelScriptDebug;
struct DECLSPEC_UUID("53159B6D-D4C4-471b-A863-5B110CA800CA") IDataModelScriptDebugClient;
struct DECLSPEC_UUID("051364DD-E449-443e-9762-FE578F4A5473") IDataModelScriptDebugStack;
struct DECLSPEC_UUID("DEC6ED5E-6360-4941-AB4C-A26409DE4F82") IDataModelScriptDebugStackFrame;
struct DECLSPEC_UUID("0F9FEED7-D045-4ac3-98A8-A98942CF6A35") IDataModelScriptDebugVariableSetEnumerator;
struct DECLSPEC_UUID("6BB27B35-02E6-47cb-90A0-5371244032DE") IDataModelScriptDebugBreakpoint;
struct DECLSPEC_UUID("39484A75-B4F3-4799-86DA-691AFA57B299") IDataModelScriptDebugBreakpointEnumerator;
struct DECLSPEC_UUID("CBB10ED3-839E-426c-9243-E23535C1AE1A") IDataModelScriptDebug2;

struct DECLSPEC_UUID("A7830646-9F0C-4a31-BA19-503F33E6C8A3") IComparableConcept;
struct DECLSPEC_UUID("C52D5D3D-609D-4d5d-8A82-46B0ACDEC4F4") IEquatableConcept;

struct DECLSPEC_UUID("2CD9906F-F1B3-4463-828A-0ADDAFE8BAAE") IActionableConcept;
struct DECLSPEC_UUID("7FC09C9F-632D-48e8-A97B-2F4F2E5C1161") IActionQueryConcept;
struct DECLSPEC_UUID("3DEC5C44-F63A-4ca6-90F0-FD5C269FDA59") IActionEnumerator;
struct DECLSPEC_UUID("1A9409F1-F0E0-4b48-9A4E-5783548FB57A") IConstructableConcept;
struct DECLSPEC_UUID("F798139E-1B2C-4077-8D87-9FA5D044F3EB") IDeconstructableConcept;

struct DECLSPEC_UUID("A754393C-4FBE-4178-8AD5-FE6079AC048D") IDebugHostFunctionIntrospection;
struct DECLSPEC_UUID("8E1CB118-AA83-409a-AAE9-C7FF78911A5F") IDebugHostFunctionIntrospection2;
struct DECLSPEC_UUID("A24E286B-891A-40fc-8A3A-89B66EDDCE57") IDebugHostFunctionIntrospection3;
struct DECLSPEC_UUID("A61ADC36-1ED5-40fe-A976-6A21CD81E811") IDebugHostFunctionLocalDetailsEnumerator;
struct DECLSPEC_UUID("89280EA8-B3B9-408c-BE16-32AB28F5C0AC") IDebugHostFunctionLocalDetails;
struct DECLSPEC_UUID("199A57B0-1967-4363-B25E-90C7E8A07F22") IDebugHostFunctionLocalDetails2;
struct DECLSPEC_UUID("026C9E81-8B9F-4d32-9606-A394EC62B045") IDebugHostFunctionLocalStorageEnumerator;
struct DECLSPEC_UUID("2F2F303B-39BE-4b6d-9BFB-4FAA49DBBD45") IDebugHostFunctionLocalStorage;
struct DECLSPEC_UUID("213B3725-36A2-45A0-9EA4-854D46D85195") IDebugHostFunctionLocalStorage2;

struct DECLSPEC_UUID("63832802-2DB3-4DE7-B76C-197AC15B5EC6") IFilteredNamespacePropertyToken;

//
// ModelObjectKind:
//
// Describes what an IModelObject is intrinsically.
//
enum ModelObjectKind
{
    // The model object is a property accessor which can be called to retrieve value, etc...
    //
    // Calling GetIntrinsicValue on the object will yield a variant in which the punkVal
    // IUnknown pointer is an IModelPropertyAccessor.
    //
    ObjectPropertyAccessor,

    // The model object is a wrapped host context (allowing such to be used as an indexer, etc...)
    //
    // Calling GetIntrinsicValue on the object will yield a variant in which the punkVal
    // IUnknown pointer is an IDebugHostContext.
    //
    ObjectContext,

    // It's a typed object within the debuggee.  It may or may not have a model associated with it.
    // If it has a model, key/value pairs may be associated.
    //
    // This object type has no "intrinsic value".  It always has a location which can be acquired.
    //
    ObjectTargetObject,

    // It's a reference to an object within the debuggee (e.g.: the object *REFERS TO* a "target int" or a
    // "target int&").  This is distinct from an object within the debuggee which is a reference (e.g.:
    // the object *IS* a "target int&").
    //
    // This allows an evaluator or other client of the model to take a "reference" to a "reference"
    // (object reference to a int&) or to take a "reference" to an object which is enregistered.
    //
    // This object type has no "intrinsic value".  It always has a location which can be acquired.
    // The underlying object can be acquired through the Dereference method.
    //
    ObjectTargetObjectReference,

    // The model object is a synthetic object (a key/value/metadata store)
    //
    // This object type has no "intrinsic value" or location.  It is purely a key/value/metadata store.
    //
    ObjectSynthetic,

    // The model object represents no value.  If a given key exists but only has a value conditionally (e.g.: it's
    // a property accessor), it can return NoValue to indicate this.  The caller should treat this appropriately (e.g.:
    // not displaying the key/value in a visualization, etc...)
    //
    // This object type has no "intrinsic value" or location.
    //
    ObjectNoValue,

    // The model object represents an error.
    //
    // This object type has no "intrinsci value" or location.  It can always be converted to a string to determine
    // the error message.
    //
    ObjectError,

    // The type is an intrinsic which is communicated through a variant (and the resulting variant type)
    //
    // Calling GetIntrinsicValue on this type will yield a variant in which the value has been packed in
    // its natural form.  String objects are packed as VT_BSTR.
    //
    ObjectIntrinsic,

    // The model object is a method which can be called.
    //
    // Calling GetIntrinsicValue on the object will yield a variant in which the punkVal
    // IUnknown pointer is an IModelMethod.
    //
    ObjectMethod,

    // The model object is a key reference.
    //
    // Calling GetIntrinsicValue on the object will yield a variant in which the punkVal
    // IUnknown pointer is an IKeyReference.
    //
    ObjectKeyReference,

};

// SymbolKind:
//
// Defines the kind of a symbol.
//
enum SymbolKind
{
    // Unspecified symbol type.
    Symbol,

    // The symbol is a module and is QI'able for IDebugHostModule
    SymbolModule,

    // The symbol is a type and is QI'able for IDebugHostType
    SymbolType,

    // The symbol is a field and is QI'able for IDebugHostField
    SymbolField,

    // The symbol is a constant and is QI'able for IDebugHostConstant
    SymbolConstant,

    // The symbol is data which is not a field of a structure and is QI'able for IDebugHostData
    SymbolData,

    // The symbol is a base class and is QI'able for IDebugHostBaseClass
    SymbolBaseClass,

    // The symbol is a public symbol and is QI'able for IDebugHostPublic
    SymbolPublic,

    // The symbol is a function symbol and is QI'able for IDebugHostData
    SymbolFunction,
};

// TypeKind:
//
// Defines the kind of a type.
//
enum TypeKind
{
    // The type is a UDT (user defined type -- a struct, class, etc...)
    //
    // The canonical form of an IModelObject which represents a UDT *value* is
    // ObjectTargetObject where the type is always kept.
    //
    TypeUDT,

    // The type is a pointer
    //
    // The canonical form of an IModelObject which represents a pointer *value* is
    // ObjectIntrinsic where the type is always kept.  The value of the pointer
    // is zero extended to 64-bits and can be retrieved with a fetch of the intrinsic
    // value of the object.  The value is packed into the VT_UI8 (ullVal) field of
    // the object's variant data.
    //
    // The base type of a pointer as returned by GetBaseType() is the type pointed to.
    //
    TypePointer,

    // The type is a member pointer
    //
    // The canonical form of an IModelObject which represents a member pointer *value* is
    // much the same as that of pointer.  The specific meaning of the intrinsic value
    // packed into the variant is compiler specific.
    //
    TypeMemberPointer,

    // The type is an array
    //
    // The canonical form of an IModelObject which represents an array *value* is
    // ObjectTargetObject where the type is always kept.  The array is always referred
    // to by a location which can be retrieved via the object's GetLocation() method.
    // The actual array is not packed into the model object.
    //
    // The base type of an array as returned by GetBaseType() is the type of each element
    // of the array.
    //
    TypeArray,

    // The type is a function
    TypeFunction,

    // **************************************************************************
    // This entry is **DEPRECATED**.
    // **************************************************************************
    //
    // The canonical form of an IModelObject which is a typedef is the same as the canonical
    // form of whatever the typedef is for.  A typedef will appear completely transparent to
    // the user of the object and the type information unless the explicit typedef methods of
    // IDebugHostType2 are utilized to query typedef information or there is an explicit data
    // model registered against the typedef.
    //
    TypeTypedef,

    // The type is an enum
    //
    // The canonical form of an IModelObject which represents an enum *value* is
    // ObjectIntrinsic where the type is always kept.  The value is packed into
    // the appropriate type in the object's variant data as described by the
    // storage type of the enumeration.
    //
    TypeEnum,

    // The type is an intrinsic (basic type)
    //
    // The canonical form of an IModelObject which represents an intrinsic *value* is
    // ObjectIntrinsic.  The value is packed into the appropriate tyep in the object's
    // variant data as indicated by the kind of intrinsic.
    //
    // Note that keeping the type information associated with the IModelObject is optional
    // if the underlying type is fully described by the variant data type (VT_*).
    //
    TypeIntrinsic,

    // The type is an array which cannot be expressed as TypeArray.
    //
    // This is due to things such as dynamic sizes, dynamic bounds, etc...  CLI arrays are
    // represented as TypeExtendedArray.
    //
    TypeExtendedArray
};

// IntrinsicKind:
//
// Defines the kind of an intrinsic (basic) type.  This is distinct from the variant type
// which carries the type.
//
enum IntrinsicKind
{
    // void
    IntrinsicVoid,

    // bool
    IntrinsicBool,

    // char
    IntrinsicChar,

    // wchar_t
    IntrinsicWChar,

    // signed int (of some size -- not necessarily 4 bytes)
    IntrinsicInt,

    // unsigned int (of some size -- not necessarily 4 bytes)
    IntrinsicUInt,

    // long (of some size)
    IntrinsicLong,

    // unsigned long (of some size)
    IntrinsicULong,

    // floating point (of some size -- not necessarily 4 bytes)
    IntrinsicFloat,

    // HRESULT
    IntrinsicHRESULT,

    // char16_t
    IntrinsicChar16,

    // char32_t
    IntrinsicChar32

};

// PointerKind:
//
// Defines the kind of a pointer type.
//
enum PointerKind
{
    // *
    PointerStandard,

    // &
    PointerReference,

    // &&
    PointerRValueReference,

    // ^
    PointerCXHat,

    // CLI reference (invisible to the user)
    PointerManagedReference
};

// CallingConventionKind:
//
// Defines the kind of calling convention of a function type.
//
enum CallingConventionKind
{
    // The calling convention is not known
    CallingConventionUnknown,

    // The calling convention is __cdecl
    CallingConventionCDecl,

    // The calling convention is fastcall
    CallingConventionFastCall,

    // The calling convention is stdcall
    CallingConventionStdCall,

    // The calling convention is syscall
    CallingConventionSysCall,

    // The calling convention is thiscall
    CallingConventionThisCall,

};

// LocationKind:
//
// Defines the location of a field or other data.
//
enum LocationKind
{
    // The field is a member and has an offset relative to the this pointer
    LocationMember,

    // The field is static and has an address
    LocationStatic,

    // The field is constant and has a value
    LocationConstant,

    // The field has no location (e.g.: it has been optimized out or was a static which was defined but not
    // declared)
    LocationNone,
};

//*************************************************
// Metadata Keys:

//
// "PreferredFormat"
//     contains a value which is from the PreferredFormat enumeration below that indicates the preferred manner in which a given
//     value should be *DISPLAYED*.  It does not affect the value itself.
//
// "PreferredRadix"
//     contains a value which indicates the preferred display radix for an integral value.  This is either 8, 10, or 16
//
// "PreferShow"
//     contains a boolean value which contains an indication of whether the element should, by default, display.
//     The default value of this metadata key is "true" for values which are not methods and "false" for values
//     which are methods.
//
// "PreferredLength"
//     contains a value which describes how many iterated elements to display by default.  
//     The default value of this metadata key is "1" for pointers and the type system defined length of any array
//     for array types.
// 
// "ActionName"
//     applicable only to object methods which take no arguments and have ObjectNoValueReturns, the presence of this
//     metadatakey indicates that the method is an action for the object which should be funneled to appropriate UI 
//     under the name given in this string metadata key.  The UI here may be a DML link (in WinDbg), a context menu,
//     or other affordance.  An example of an action is "Switch To" for a thread which changes the UI focus to
//     the given thread.
//
// "ActionDescription"
//     applicable only where the "ActionName" key is present, this is a string value which gives tooltip style
//     help for an action.
//
// "ActionIsDefault"
//     applicable only where the "ActionName" key is present, this is a boolean value which describes whether
//     the action is a default action or not.  The default action for an object may be funneled to additional
//     UI.  By default, the value of this key is "false".  Only a single action method on any given object may
//     be marked as the default action.
//
// "PreferAutoExpand"
//     contains an unsigned value which describes whether the element should be expanded automatically if
//     it is a child of some other object and the recursion level is high enough.  The default value for this
//     key is "true".  If this key is specified as "false", the object **WILL NOT** expand in a console view unless
//     it is the root element being displayed.
//
// "PreferredExpansionDepth"
//     contains an unsigned value which describes how far the element should be expanded if not otherwise specified
//     by a host command or other UI affordance.
//
// "PreferTabularFormat"
//     contains a boolean value which describes whether the value should be displayed in a tabular form instead
//     of a hierarchical form
//

//
// PreferredFormat
//
// Predefined values of the "PreferredFormat" key which may appear as the metadata on a returned key value.  This indicates
// the preferred DISPLAY FORMAT for a given value.
//
enum PreferredFormat
{
    // There is no preferred format
    FormatNone,

    // The preferred format is a single character as '*'
    FormatSingleCharacter,

    // The preferred format is a quoted 8-bit string
    FormatQuotedString,

    // The preferred format is a non-quoted 8-bit string
    FormatString,

    // The preferred format is a quoted Unicode (UTF-16) string
    FormatQuotedUnicodeString,

    // The preferred format is a non-quoted Unicode (UTF-16) string
    FormatUnicodeString,

    // The preferred format is a quoted UTF-8 string
    FormatQuotedUTF8String,

    // The preferred format is a non-quoted UTF-8 string
    FormatUTF8String,

    // The preferred format is a quoted BSTR
    FormatBSTRString,

    // The preferred format is a quoted WinRT HSTRING
    FormatQuotedHString,

    // The preferred format is a non-quoted WinRT HSTRING
    FormatHString,

    // The preferred format is the raw (native) type
    FormatRaw,

    // The preferred format is the enum name only
    FormatEnumNameOnly,

    // The preferred format is the quoted string with escaped characters
    FormatEscapedStringWithQuote,

    // The preferred format is a non-quoted Unicode (UTF-32) string
    FormatUTF32String,

    // The preferred format is a quoted Unicode (UTF-32) string
    FormatQuotedUTF32String
};

//
// IHostDataModelAccess:
//
// An interface *suggested* on the per-host extensibility mechanism to get from the host extensibility
// mechanism to the model based one.  Extensions which are written to a host-specific API set can query
// this mechanism to get to the data model and create host-agnostic extensions.
//
// As an example, DbgEng based extensions can query for this interface from any IDebug* (Client/Control/etc...)
// interface to get to the model and access model APIs.  Such extensions are hybrid (they are still specific
// to a particular host but contain portions that may be factored out later for a general model based extension)
//
// This is the **ONLY** interface in this set of APIs which is not intended to be host agnostic.
//
#undef INTERFACE
#define INTERFACE IHostDataModelAccess
DECLARE_INTERFACE_(IHostDataModelAccess, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IKeyStore

    // GetDataModel():
    //
    // Returns the core interfaces to the data model.  A hybrid extension can query its host's
    // extensibility interfaces for IHostDataModelAccess and subsequently call this method to
    // get to the data model interfaces.
    //
    // There is no guarantee that any particular host supports this interface.
    //
    STDMETHOD(GetDataModel)(
        THIS_
        _COM_Outptr_ IDataModelManager** manager,
        _COM_Outptr_ IDebugHost** host
        ) PURE;
};

//
// IKeyStore:
//
// An interface which represents a set of key/value/metadata tuples.  This is primarily the
// interface by which metadata is represented.
//
#undef INTERFACE
#define INTERFACE IKeyStore
DECLARE_INTERFACE_(IKeyStore, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IKeyStore

    // GetKey():
    //
    // If the store or its parent has a key named according to the argument 'key', this will
    // return the value of that key and, optionally, any metadata associated with that key.
    //
    // If the key is a property accessor, this API will return the property accessor itself.  It is the responsibility
    // of the caller to resolve the property by fetching the IModelPropertyAccessor interface and calling into it
    // passing the appropriate context object.
    //
    STDMETHOD(GetKey)(
        THIS_
        _In_ PCWSTR key,
        _COM_Errorptr_opt_ IModelObject** object,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    // SetKey():
    //
    // If the store or its parent has a key named according to the argument 'key', this will
    // set the value of that key and optionally its associated metadata.
    //
    // If the key is a property accessor, this will not set the value of the property.  It will explicitly
    // overwrite the property with the new value.
    //
    // This is the only method which is capable of creating a new key.  If a new key is to be created, this
    // will create the key on the store where SetKey was called.  It will not create a key on the parent store.
    //
    STDMETHOD(SetKey)(
        THIS_
        _In_ PCWSTR key,
        _In_opt_ IModelObject* object,
        _In_opt_ IKeyStore* metadata
        ) PURE;

    // GetKeyValue():
    //
    // If the store or its parent has a key named according to the argument 'key', this will
    // return the value of that key and, optionally, any metadata associated with that key.
    //
    // If the key is a property accessor, this API will fetch the value underlying the property and return it.
    // It will not return the IModelPropertyAccessor interface for the property.
    //
    STDMETHOD(GetKeyValue)(
        THIS_
        _In_ PCWSTR key,
        _COM_Errorptr_opt_ IModelObject** object,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    // SetKeyValue():
    //
    // If the store or its parent has a key named according to the argument 'key', this will
    // set the value of that key.
    //
    // If the key is a property accessor, this API will call the underlying property to set the value of the
    // property.  Note that some properties are read-only and, as such, this API may return a failure if called
    // on such a property.  If the key is not a property accessor, it will overwrite the value of the key directly.
    //
    // This method will never create a new key named 'key'.
    //
    STDMETHOD(SetKeyValue)(
        THIS_
        _In_ PCWSTR key,
        _In_ IModelObject* object
        ) PURE;

    // ClearKeys():
    //
    // This will clear all keys on the store but *NOT* on its parent.
    //
    STDMETHOD(ClearKeys)(
        THIS
        ) PURE;

};

// RawSearchFlags:
//
// Flags to GetRawValue/EnumerateRawValues
//
enum RawSearchFlags
{
    // There are no search flags
    RawSearchNone = 0x00000000,

    // Indicates that the search should not recurse to base children (e.g.: base classes).  Only names/types
    // which are in the object itself should be returned.
    RawSearchNoBases = 0x00000001,
};

//
// IModelObject:
//
// The core object model interface.  All object instances that the model can represent are directly referred
// to by the IModelObject interface.
//
// This interface is never directly implemented by a client.
//
#undef INTERFACE
#define INTERFACE IModelObject
DECLARE_INTERFACE_(IModelObject, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IModelObject

    // Client Methods:

    // GetContext():
    //
    // Gets the host (debugger) context that is associated with this object.
    //
    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_result_maybenull_ IDebugHostContext** context
        ) PURE;

    // GetKind():
    //
    // Gets the kind of this object.  This indicates the kind of object (e.g.: a boxed intrinsic value,
    // an object within the address space of the debug target, a synthetic object created by the
    // debugger, an error, etc...).  This does not indicate the language type of the object.
    //
    STDMETHOD(GetKind)(
        THIS_
        _Out_ ModelObjectKind *kind
        ) PURE;

    // GetIntrinsicValue():
    //
    // If the object has an intrinsic value because it is a boxed intrinsic, a pointer, or a construct
    // which has an associated IUnknown, this will return the intrinsic value of the object as a
    // VARIANT.
    //
    // If the object does not have an intrinsic value (e.g.: it is a completely synthetic object), calling
    // this API will result in an error.
    //
    STDMETHOD(GetIntrinsicValue)(
        THIS_
        _Out_ VARIANT* intrinsicData
        ) PURE;

    // GetIntrinsicValueAs():
    //
    // If the object has an intrinsic value because it is a boxed intrinsic, a pointer, or a construct
    // which has an associated IUnknown, this will return the intrinsic value of the object converted
    // to the variant type passed in the 'vt' argument.  If the value cannot be converted to the given
    // variant type, this will return an error.
    //
    STDMETHOD(GetIntrinsicValueAs)(
        THIS_
        _In_ VARTYPE vt,
        _Out_ VARIANT* intrinsicData
        ) PURE;

    // GetKeyValue():
    //
    // If the object or one of its parent models has a key named according to the argument 'key', this will
    // return the value of that key and, optionally, any metadata associated with that key.
    //
    // If the key is a property accessor, this API will fetch the value underlying the property and return it.
    // It will not return the IModelPropertyAccessor interface for the property.
    //
    STDMETHOD(GetKeyValue)(
        THIS_
        _In_ PCWSTR key,
        _COM_Errorptr_opt_ IModelObject** object,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    // SetKeyValue():
    //
    // If the object or one of its parent models has a key named according to the argument 'key', this will
    // set the value of that key.
    //
    // If the key is a property accessor, this API will call the underlying property to set the value of the
    // property.  Note that some properties are read-only and, as such, this API may return a failure if called
    // on such a property.  If the key is not a property accessor, it will overwrite the value of the key directly.
    //
    // This method will never create a new key named 'key'.
    //
    STDMETHOD(SetKeyValue)(
        THIS_
        _In_ PCWSTR key,
        _In_opt_ IModelObject* object
        ) PURE;

    // EnumerateKeyValues():
    //
    // This returns an enumerator which will enumerate all keys and their associated values on this object and
    // any of its parent values.  Note that if any of the enumerated values are property accessors, the underlying
    // property will be fetched and returned by the enumerator.  This will never return an object which is an
    // IModelPropertyAccessor.
    //
    STDMETHOD(EnumerateKeyValues)(
        THIS_
        _COM_Outptr_ IKeyEnumerator** enumerator
        ) PURE;

    // GetRawValue():
    //
    // If the object represents an object within the address space of the debug target which has children (e.g.: a user
    // defined type such as a struct or class), this will attempt to return the child named 'name'.  The kind of child
    // being sought (e.g.: field, base class, etc...) should be passed in the 'kind' argument.
    //
    // If no child named 'name' can be found, this method will fail.  If there is an ambiguity presented by 'name', an
    // error will be returned.
    //
    STDMETHOD(GetRawValue)(
        THIS_
        _In_ SymbolKind kind,
        _In_ PCWSTR name,
        _In_ ULONG searchFlags,
        _COM_Errorptr_ IModelObject** object
        ) PURE;

    // EnumerateRawValues():
    //
    // This returns an enumerator which will enumerate all raw (native) children of the object of the kind 'kind'.
    //
    STDMETHOD(EnumerateRawValues)(
        THIS_
        _In_ SymbolKind kind,
        _In_ ULONG searchFlags,
        _COM_Outptr_ IRawEnumerator** enumerator
        ) PURE;

    // Dereference():
    //
    // If the object in question is something which is dereferencable (e.g.: a pointer or reference), calling this API will
    // return the dereferenced object.  Calling this API on a non-dereferencable object will result in an error being returned.
    //
    STDMETHOD(Dereference)(
        THIS_
        _COM_Errorptr_ IModelObject** object
        ) PURE;

    // TryCastToRuntimeType():
    //
    // If the debug host can identify the runtime type of this object (e.g.: the most derived class), calling this method
    // will return that object.  If the debug host cannot identify such runtime type or the present object is the runtime
    // type, the output object 'runtimeTypedObject' will be this object.
    // A given object can specify it's "runtime type" by implementing the IPreferredRuntimeTypeConcept concept. If that concept
    // is not implemented the debug host method will be used.
    //
    // Note that it is possible for this method to fail for a variety of reasons (e.g.: inability to locate symbols, etc...).
    //
    STDMETHOD(TryCastToRuntimeType)(
        THIS_
        _COM_Errorptr_ IModelObject** runtimeTypedObject
        ) PURE;

    // GetConcept():
    //
    // If the object or one of its parent models has a concept identified by the IID contained in the 'conceptId'
    // argument, this will return the concept interface in the 'conceptInterface' argument.  This is the only valid method
    // of acquiring any concept interface.  Concept interfaces should not attempt to be accessed via a QueryInterface
    // call.
    //
    STDMETHOD(GetConcept)(
        THIS_
        _In_ REFIID conceptId,
        _COM_Outptr_ IUnknown** conceptInterface,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** conceptMetadata
        ) PURE;

    // GetLocation():
    //
    // If the object represents an object in the address space of the debug target, this will return the address
    // of the object.
    //
    // It is a failure to call this API on any type other than one which declares that it is a TargetObjectType via the
    // GetKind method.
    //
    STDMETHOD(GetLocation)(
        THIS_
        _Out_ Location* location
        ) PURE;

    // GetTypeInfo():
    //
    // If the object has an associated type (native language type), this will return an interface representing that type.
    // Note that if a given type is inherently described by the type of a VARIANT structure, it will not have associated
    // type information.
    //
    // If the object does not have associated type information, this API will return 'nullptr'.  This is not a failure.
    //
    STDMETHOD(GetTypeInfo)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    // GetTargetInfo():
    //
    // If the object represents an object in the address space of the debug target, this will return both the address
    // of the object and the type (native language type) of the object.  The type returned from this method is identical
    // to the one returned from the GetTypeInfo method.
    //
    // It is a failure to call this API on any type other than one which declares that it is a TargetObjectType via the
    // GetType method.
    //
    STDMETHOD(GetTargetInfo)(
        THIS_
        _Out_ Location* location,
        _Out_ IDebugHostType** type
        ) PURE;

    // Advanced clients:

    // GetNumberOfParentModels():
    //
    // Returns the number of parent models of this object.  An object may have zero or more parent models associated with it.
    // If the object does not have a given key or concept when queried, such calls are passed to all parent models in
    // linear order to satisfy the request.
    //
    STDMETHOD(GetNumberOfParentModels)(
        _Out_ ULONG64* numModels
        ) PURE;

    // GetParentModel():
    //
    // Returns the 'i'-th parent model of this object and, optionally, the adjusted context object associated with that
    // parent model.
    //
    // If the object does not have a given key or concept when queried, such calls are passed to all parent models in
    // linear order to satisfy the request.  A parent model may also adjust the context (effective this pointer) object.  If
    // the model performs such adjustment, the adjusted object is returned in the 'contextObject' argument.  If not,
    // nullptr is returned in the 'contextObject' argument.
    //
    STDMETHOD(GetParentModel)(
        _In_ ULONG64 i,
        _COM_Outptr_ IModelObject **model,
        _COM_Outptr_result_maybenull_ IModelObject **contextObject
        ) PURE;

    // AddParentModel():
    //
    // Adds a new parent model to this object.  If the object does not have a given key or concept when queried, such calls are
    // passed to all parent models in linear order to satisfy the request.
    //
    // If the parent model needs to adjust the context (effective this pointer) object so that property accessors and other
    // concept interfaces on the parent model receive a different context pointer than the object itself, such an adjusted context
    // can be passed in the 'contextObject' argument.  If no adjustment is necessary, nullptr should be passed.  Note that it
    // is perfectly legitimate for the object passed in 'contextObject' to be a property accessor.  In that case, the property
    // will always be resolved before being given to any caller of GetParentModel or passed to any other accessor or concept.
    //
    // If this model should take precedence over any other parent models, the 'override' argument should be 'true'; otherwise,
    // it should be 'false' and the new model will be added last in search order.
    //
    STDMETHOD(AddParentModel)(
        THIS_
        _In_ IModelObject* model,
        _In_opt_ IModelObject* contextObject,
        _In_ bool override) PURE;

    // RemoveParentModel():
    //
    // Removes a parent model as indicated by the 'model' argument from the list of parent models of this object.
    //
    STDMETHOD(RemoveParentModel)(
        THIS_
        _In_ IModelObject* model
        ) PURE;

    // GetKey():
    //
    // If the object or one of its parent models has a key named according to the argument 'key', this will
    // return the value of that key and, optionally, any metadata associated with that key.
    //
    // If the key is a property accessor, this API will return the property accessor itself.  It is the responsibility
    // of the caller to resolve the property by fetching the IModelPropertyAccessor interface and calling into it
    // passing the appropriate context object.
    //
    STDMETHOD(GetKey)(
        THIS_
        _In_ PCWSTR key,
        _COM_Errorptr_opt_ IModelObject** object,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    // GetKeyReference():
    //
    // If the object or one of its parent models has a key named according to the argument 'key', this will
    // return a reference to that key (and optionally the *present* metadata associated with that key).
    //
    STDMETHOD(GetKeyReference)(
        THIS_
        _In_ PCWSTR key,
        _COM_Errorptr_opt_ IModelObject** objectReference,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    // SetKey():
    //
    // If the object or one of its parent models has a key named according to the argument 'key', this will
    // set the value of that key and optionally its associated metadata.
    //
    // If the key is a property accessor, this will not set the value of the property.  It will explicitly
    // overwrite the property with the new value.
    //
    // This is the only method which is capable of creating a new key.  If a new key is to be created, this
    // will create the key on the object where SetKey was called.  It will not create a key on a parent
    // model.
    //
    STDMETHOD(SetKey)(
        THIS_
        _In_ PCWSTR key,
        _In_opt_ IModelObject* object,
        _In_opt_ IKeyStore* metadata
        ) PURE;

    // ClearKeys():
    //
    // This will clear all keys on the object but *NOT* on the parents that are attached to the object.
    //
    STDMETHOD(ClearKeys)(
        THIS
        ) PURE;

    // EnumerateKeys():
    //
    // This returns an enumerator which will enumerate all keys and their associated values on this object and
    // any of its parent values.  Note that if any of the enumerated values are property accessors, the underlying
    // value will not be fetched.  The enumerator will instead return the property accessor object.  It is the responsibility
    // of the user of the enumerator to resolve the property value by fetching the IModelPropertyAccessor interface and
    // calling into it passing the appropriate context object.
    //
    STDMETHOD(EnumerateKeys)(
        THIS_
        _COM_Outptr_ IKeyEnumerator** enumerator
        ) PURE;

    // EnumerateKeyReferences():
    //
    // This returns an enumerator which will enumerate all keys on this object and any of its parent values.  This will
    // return references to those keys rather than the keys themselves.
    //
    STDMETHOD(EnumerateKeyReferences)(
        THIS_
        _COM_Outptr_ IKeyEnumerator** enumerator
        ) PURE;

    // SetConcept():
    //
    // This sets a concept interface identified by the 'conceptId' argument on the object.  Such concept interfaces
    // can make the object convertible to a string, iterable, indexable, etc...
    //
    STDMETHOD(SetConcept)(
        THIS_
        _In_ REFIID conceptId,
        _In_ IUnknown* conceptInterface,
        _In_opt_ IKeyStore* conceptMetadata
        ) PURE;

    // ClearConcepts():
    //
    // This clears all concepts on the object but *NOT* on the parents that are attached to the object.
    //
    STDMETHOD(ClearConcepts)(
        THIS
        ) PURE;

    // GetRawReference():
    //
    // If the object represents an object within the address space of the debug target which has children (e.g.: a user
    // defined type such as a struct or class), this will attempt to return a reference to the child named 'name'.  The type
    // of child being sought (e.g.: field, base class, etc...) should be passed in the 'type' argument.
    //
    // If no child named 'name' can be found, this method will fail.  If there is an ambiguity presented by 'name', an
    // error will be returned.
    //
    STDMETHOD(GetRawReference)(
        THIS_
        _In_ SymbolKind kind,
        _In_ PCWSTR name,
        _In_ ULONG searchFlags,
        _COM_Errorptr_ IModelObject** object
        ) PURE;

    // EnumerateRawReferences():
    //
    // This returns an enumerator which will enumerate references to all raw (native) children of the object of the type 'type'.
    //
    STDMETHOD(EnumerateRawReferences)(
        THIS_
        _In_ SymbolKind kind,
        _In_ ULONG searchFlags,
        _COM_Outptr_ IRawEnumerator** enumerator
        ) PURE;

    // SetContextForDataModel():
    //
    // This method allows a parent data model which is linked to multiple instance objects to add cache information to each
    // instance object.  The model object for the data model is passed in the 'dataModelObject' interface and an IUnknown
    // which represents the information to cache is passed in the 'context' argument.  When the instance object destructs,
    // it will release the cached information.
    //
    STDMETHOD(SetContextForDataModel)(
        THIS_
        _In_ IModelObject* dataModelObject,
        _In_ IUnknown* context
        ) PURE;

    // GetContextForDataModel():
    //
    // If a parent model has associated cached information with an instance through a prior call to SetContextForDataModel,
    // this method allows the parent model to fetch the previously stored information.  The model object for the data model
    // is passed in the 'dataModelObject' argument.
    //
    // Note that a lack of cache information is not a failure.  The returned 'context' argument will simply be nullptr in this
    // case.
    //
    STDMETHOD(GetContextForDataModel)(
        THIS_
        _In_ IModelObject* dataModelObject,
        _Out_ IUnknown** context
        ) PURE;

    // Compare():
    //
    // Compares this object against another object.  Returns -1 (this < other), 0 (this == other), or 1 (this > other).
    // If the objects cannot be compared, a failure occurs.
    //
    // Only intrinsic values may be compared using this method.
    //
    STDMETHOD(Compare)(
        THIS_
        _In_ IModelObject* other,
        _COM_Outptr_opt_result_maybenull_ IModelObject **ppResult
        ) PURE;

    // IsEqualTo():
    //
    // Compares this object against another object.  Returns true or false as to whether the objects are equal.
    // For objects which have an ordering, this returning true is equivalent to Compare(...) returning 0.  For objects
    // that have no ordering but are equatable, Compare(...) will fail, but this will not.
    //
    // Note that this is a value based comparison as defined by the type.
    //
    STDMETHOD(IsEqualTo)(
        THIS_
        _In_ IModelObject* other,
        _Out_ bool* equal
        ) PURE;

};

//
// IModelObject2:
//
// This is the second version of the core object model interface.  All object instances that the model can represent are directly referred
// to by the IModelObject interface.
//
// This interface is never directly implemented by a client.
//
#undef INTERFACE
#define INTERFACE IModelObject2
DECLARE_INTERFACE_(IModelObject2, IModelObject)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IModelObject

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_result_maybenull_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(GetKind)(
        THIS_
        _Out_ ModelObjectKind *kind
        ) PURE;

    STDMETHOD(GetIntrinsicValue)(
        THIS_
        _Out_ VARIANT* intrinsicData
        ) PURE;

    STDMETHOD(GetIntrinsicValueAs)(
        THIS_
        _In_ VARTYPE vt,
        _Out_ VARIANT* intrinsicData
        ) PURE;

    STDMETHOD(GetKeyValue)(
        THIS_
        _In_ PCWSTR key,
        _COM_Errorptr_opt_ IModelObject** object,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    STDMETHOD(SetKeyValue)(
        THIS_
        _In_ PCWSTR key,
        _In_opt_ IModelObject* object
        ) PURE;

    STDMETHOD(EnumerateKeyValues)(
        THIS_
        _COM_Outptr_ IKeyEnumerator** enumerator
        ) PURE;

    STDMETHOD(GetRawValue)(
        THIS_
        _In_ SymbolKind kind,
        _In_ PCWSTR name,
        _In_ ULONG searchFlags,
        _COM_Errorptr_ IModelObject** object
        ) PURE;

    STDMETHOD(EnumerateRawValues)(
        THIS_
        _In_ SymbolKind kind,
        _In_ ULONG searchFlags,
        _COM_Outptr_ IRawEnumerator** enumerator
        ) PURE;

    STDMETHOD(Dereference)(
        THIS_
        _COM_Errorptr_ IModelObject** object
        ) PURE;

    STDMETHOD(TryCastToRuntimeType)(
        THIS_
        _COM_Errorptr_ IModelObject** runtimeTypedObject
        ) PURE;

    STDMETHOD(GetConcept)(
        THIS_
        _In_ REFIID conceptId,
        _COM_Outptr_ IUnknown** conceptInterface,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** conceptMetadata
        ) PURE;

    STDMETHOD(GetLocation)(
        THIS_
        _Out_ Location* location
        ) PURE;

    STDMETHOD(GetTypeInfo)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetTargetInfo)(
        THIS_
        _Out_ Location* location,
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetNumberOfParentModels)(
        _Out_ ULONG64* numModels
        ) PURE;

    STDMETHOD(GetParentModel)(
        _In_ ULONG64 i,
        _COM_Outptr_ IModelObject **model,
        _COM_Outptr_result_maybenull_ IModelObject **contextObject
        ) PURE;

    STDMETHOD(AddParentModel)(
        THIS_
        _In_ IModelObject* model,
        _In_opt_ IModelObject* contextObject,
        _In_ bool override) PURE;

    STDMETHOD(RemoveParentModel)(
        THIS_
        _In_ IModelObject* model
        ) PURE;

    STDMETHOD(GetKey)(
        THIS_
        _In_ PCWSTR key,
        _COM_Errorptr_opt_ IModelObject** object,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    STDMETHOD(GetKeyReference)(
        THIS_
        _In_ PCWSTR key,
        _COM_Errorptr_opt_ IModelObject** objectReference,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    STDMETHOD(SetKey)(
        THIS_
        _In_ PCWSTR key,
        _In_opt_ IModelObject* object,
        _In_opt_ IKeyStore* metadata
        ) PURE;

    STDMETHOD(ClearKeys)(
        THIS
        ) PURE;

    STDMETHOD(EnumerateKeys)(
        THIS_
        _COM_Outptr_ IKeyEnumerator** enumerator
        ) PURE;

    STDMETHOD(EnumerateKeyReferences)(
        THIS_
        _COM_Outptr_ IKeyEnumerator** enumerator
        ) PURE;

    STDMETHOD(SetConcept)(
        THIS_
        _In_ REFIID conceptId,
        _In_ IUnknown* conceptInterface,
        _In_opt_ IKeyStore* conceptMetadata
        ) PURE;

    STDMETHOD(ClearConcepts)(
        THIS
        ) PURE;

    STDMETHOD(GetRawReference)(
        THIS_
        _In_ SymbolKind kind,
        _In_ PCWSTR name,
        _In_ ULONG searchFlags,
        _COM_Errorptr_ IModelObject** object
        ) PURE;

    STDMETHOD(EnumerateRawReferences)(
        THIS_
        _In_ SymbolKind kind,
        _In_ ULONG searchFlags,
        _COM_Outptr_ IRawEnumerator** enumerator
        ) PURE;

    STDMETHOD(SetContextForDataModel)(
        THIS_
        _In_ IModelObject* dataModelObject,
        _In_ IUnknown* context
        ) PURE;

    STDMETHOD(GetContextForDataModel)(
        THIS_
        _In_ IModelObject* dataModelObject,
        _Out_ IUnknown** context
        ) PURE;

    STDMETHOD(Compare)(
        THIS_
        _In_ IModelObject* other,
        _COM_Outptr_opt_result_maybenull_ IModelObject **ppResult
        ) PURE;

    STDMETHOD(IsEqualTo)(
        THIS_
        _In_ IModelObject* other,
        _Out_ bool* equal
        ) PURE;

    //*************************************************
     // IModelObject2
     //

    // EnumerateOwnKeyValues():
    //
    // This returns an enumerator which will enumerate all keys and their associated values on this object (but not on
    // any of its parent values).  Note that if any of the enumerated values are property accessors, the underlying
    // property will be fetched and returned by the enumerator.  This will never return an object which is an
    // IModelPropertyAccessor.
    //
    STDMETHOD(EnumerateOwnKeyValues)(
        THIS_
        _COM_Outptr_ IKeyEnumerator** ppEnumerator
        ) PURE;

    // EnumerateOwnKeys():
    //
    // This returns an enumerator which will enumerate all keys and their associated values on this object (but not on
    // any of its parent values). Note that if any of the enumerated values are property accessors, the underlying
    // value will not be fetched. The enumerator will instead return the property accessor object.  It is the responsibility
    // of the user of the enumerator to resolve the property value by fetching the IModelPropertyAccessor interface and
    // calling into it passing the appropriate context object.
    //
    STDMETHOD(EnumerateOwnKeys)(
        THIS_
        _COM_Outptr_ IKeyEnumerator** ppEnumerator
        ) PURE;

    // EnumerateOwnKeyReferences():
    //
    // This returns an enumerator which will enumerate all keys on this object (but not on any of its parent values).  This will
    // return references to those keys rather than the keys themselves.
    //
    STDMETHOD(EnumerateOwnKeyReferences)(
        THIS_
        _COM_Outptr_ IKeyEnumerator** ppEnumerator
        ) PURE;
};

//
// IDataModelManager:
//
// The core interface for the data model manager.  This is the interface by which new objects are created,
// intrinsic values are boxed and unboxed, and models are registered for types.
//
// This interface is never directly implemented by a client.
//
#undef INTERFACE
#define INTERFACE IDataModelManager
DECLARE_INTERFACE_(IDataModelManager, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelManager

    // Close():
    //
    // Cleanup method for the data model manager.  This **MUST** be called when a host is shutting down the data model
    // manager prior to releasing its reference to the manager.
    //
    STDMETHOD(Close)(
        THIS
        ) PURE;


    // CreateNoValue():
    //
    // Creates an object which represents nothing.  In essence, this appears like "void".  A property
    // which does not exist for a particular instance of an object may return "NoValue" instead of an error.
    // A parent model which is a property accessor but only applies to some objects may return "NoValue"
    // instead of an error.
    //
    // "NoValue" objects will not be displayed by many clients.
    //
    STDMETHOD(CreateNoValue)(
        THIS_
        _Out_ IModelObject** object
        ) PURE;

    // CreateErrorObject():
    //
    // Creates an object which encapsulates a failure (code and message).
    //
    STDMETHOD(CreateErrorObject)(
        THIS_
        _In_ HRESULT hrError,
        _In_opt_ PCWSTR pwszMessage,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    // CreateTypedObject():
    //
    // Creates an object within the address space of the host at the supplied location.  If a particular
    // host context is passed, the object will acquire that context; otherwise -- the object will inherit
    // its context from that of the supplied type.
    //
    // If the given object location is an *offset* from a location fetched from another object (e.g.: it is
    // a field of that object), the context supplied here should be explicitly passed from object where
    // that location came from.
    //
    // CreateTypedObject may return an error object in the output argument even in the case of failure.
    //
    STDMETHOD(CreateTypedObject)(
        THIS_
        _In_opt_ IDebugHostContext* context,
        _In_ Location objectLocation,
        _In_ IDebugHostType* objectType,
        _COM_Errorptr_ IModelObject** object
        ) PURE;

    // CreateTypedObjectReference():
    //
    // Creates a reference to an object within the address space of the host.  This is a reference as defined
    // by the model API, not a reference as defined by the language.
    //
    // In respects other than creating a reference, this behaves as per CreateTypedObject.
    //
    STDMETHOD(CreateTypedObjectReference)(
        THIS_
        _In_opt_ IDebugHostContext* context,
        _In_ Location objectLocation,
        _In_ IDebugHostType* objectType,
        _COM_Errorptr_ IModelObject** object
        ) PURE;

    // CreateSyntheticObject():
    //
    // Creates an empty synthetic object (a store of key/value/metadata tuples).
    //
    STDMETHOD(CreateSyntheticObject)(
        THIS_
        _In_opt_ IDebugHostContext* context,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    // CreateDataModelObject():
    //
    // Creates a synthetic object designed to be a data model.  This method is a convenience for the caller.
    // The resulting syntehtic object is an empty synthetic object onto which the supplied data model concept
    // has been added via SetConcept().
    //
    STDMETHOD(CreateDataModelObject)(
        THIS_
        _In_ IDataModelConcept* dataModel,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    // CreateIntrinsicObject():
    //
    // Creates an object representing an intrinsic value.  In some language parlences, this would be called
    // "boxing" a value.
    //
    // The full type of the value is defined by the VARIANT in which it is carried.  For values which have
    // ancillary type information (e.g.: enumerations), CreateTypedIntrinsicObject can be used.
    //
    // NOTE: 128-bit integers are represented in 2-sized, 1-dimensional SAFEARRAYs:
    //          VT_ARRAY | VT_UI8 is the VARTYPE for 128-bit unsigned integers.
    //          VT_ARRAY | VT_I8 is the VARTYPE for 128-bit signed integers.
    //       The first element in the array corresponds to the low 64 bits of the 128-bit integer,
    //       while the second corresponds to the high 64 bits. 
    STDMETHOD(CreateIntrinsicObject)(
        THIS_
        _In_ ModelObjectKind objectKind,
        _In_ VARIANT* intrinsicData,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    // CreateTypedIntrinsicObject():
    //
    // Similar to CreateIntrinsicObject(), this creates a value object which has ancillary type information.
    // Ancillary type information is used for enumerations, pointer types, and other things which have an
    // inherit value but by which that value is interpreted differently than just the bits of the value.
    //
    STDMETHOD(CreateTypedIntrinsicObject)(
        THIS_
        _In_ VARIANT* intrinsicData,
        _In_ IDebugHostType* type,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    // GetModelForTypeSignature():
    //
    // Returns the model currently registered for a given type signature.  Such registration can be performed
    // by calling RegisterModelForTypeSignature().
    //
    STDMETHOD(GetModelForTypeSignature)(
        THIS_
        _In_ IDebugHostTypeSignature* typeSignature,
        _Out_ IModelObject** dataModel
        ) PURE;

    // GetModelForType():
    //
    // Returns the model (registered via RegisterModelForTypeSignature) whose type signature is the best
    // match for the given type.  In addition to returning the registered model, the type signature
    // and a list of wildcard matches between the signature and the actual type is also returned.
    //
    STDMETHOD(GetModelForType)(
        THIS_
        _In_ IDebugHostType* type,
        _COM_Outptr_ IModelObject** dataModel,
        _COM_Outptr_opt_ IDebugHostTypeSignature** typeSignature,
        _COM_Outptr_opt_ IDebugHostSymbolEnumerator** wildcardMatches
        ) PURE;

    // RegisterModelForTypeSignature():
    //
    // Registers a model for a given type signature.  Any object which has a type that matches the signature
    // will have the inpassed model applied as a parent model of the object when the object is created.
    //
    STDMETHOD(RegisterModelForTypeSignature)(
        THIS_
        _In_ IDebugHostTypeSignature* typeSignature,
        _In_ IModelObject* dataModel
        ) PURE;

    // UnregisterModelForTypeSignature():
    //
    // Undoes a prior call to RegisterModelForTypeSignature().  Newly created objects of the type which matches
    // the supplied signature will no longer have the supplied data model applied as a parent model.
    //
    // Note that calling this method has no effect on pre-existing objects which matched the signature.
    //
    STDMETHOD(UnregisterModelForTypeSignature)(
        THIS_
        _In_ IModelObject* dataModel,
        _In_opt_ IDebugHostTypeSignature* typeSignature
        ) PURE;

    // RegisterExtensionForTypeSignature():
    //
    // Registers an extension for a given type signature.  Any object which has a type signature that matches the signature
    // will have the inpassed model applied as a parent model of the object when the object is created.
    //
    // Unlike RegisterModelForTypeSignature -- which affects the single CANONICAL model for a given type signature
    // and can have no ambiguous registrations -- any number of extensions can be registered against a type signature
    // (or multiple ambiguous type signatures).  All such extensions will be automatically attached.
    //
    STDMETHOD(RegisterExtensionForTypeSignature)(
        THIS_
        _In_ IDebugHostTypeSignature* typeSignature,
        _In_ IModelObject* dataModel
        ) PURE;

    // UnregisterExtensionForTypeSignature():
    //
    // Behaves as UnregisterModelForTypeSignature excepting that it undoes RegisterExtensionForTypeSignature.
    //
    STDMETHOD(UnregisterExtensionForTypeSignature)(
        THIS_
        _In_ IModelObject* dataModel,
        _In_opt_ IDebugHostTypeSignature* typeSignature
        ) PURE;

    // CreateMetadataStore():
    //
    // Creates a new key store for metadata.  Any tagging of metadata onto key names is done by creating such
    // an object, filling it, and passing it to SetKey().
    //
    STDMETHOD(CreateMetadataStore)(
        THIS_
        _In_opt_ IKeyStore* parentStore,
        _COM_Outptr_ IKeyStore** metadataStore
        ) PURE;

    // GetRootNamespace():
    //
    // Gets the root namespace which is managed by the data model.  A host and its extensions have full control
    // of what goes into this namespace.
    //
    STDMETHOD(GetRootNamespace)(
        THIS_
        _COM_Outptr_ IModelObject** rootNamespace
        ) PURE;

    // RegisterNamedModel():
    //
    // Associates a given data model with a well defined name.  This name can then be used to look up the data model.
    // All of the data models which the model itself creates are well named and registered via this mechanism.
    //
    STDMETHOD(RegisterNamedModel)(
        THIS_
        _In_ PCWSTR modelName,
        _In_ IModelObject *modeObject
        ) PURE;

    // UnregisterNamedModel():
    //
    // Removes the registration of a given name.  This undoes a prior call to RegisterNamedModel().
    //
    STDMETHOD(UnregisterNamedModel)(
        THIS_
        _In_ PCWSTR modelName
        ) PURE;

    // AcquireNamedModel():
    //
    // This looks up a well known model name and returns the data model registered by that name.  Note that if
    // there is no model registered by the supplied name, a stub will be created and returned to the caller.
    // Anything added to the stub will be added to the real object at the time a registration is made.
    //
    STDMETHOD(AcquireNamedModel)(
        THIS_
        _In_ PCWSTR modelName,
        _COM_Outptr_ IModelObject **modelObject
        ) PURE;
};

//
// IDataModelManager2:
//
// The second version of the interface for the data model manager.  This is the interface by which new objects are created,
// intrinsic values are boxed and unboxed, and models are registered for types.
//
// This interface is never directly implemented by a client.
//
#undef INTERFACE
#define INTERFACE IDataModelManager2
DECLARE_INTERFACE_(IDataModelManager2, IDataModelManager)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelManager

    STDMETHOD(Close)(
        THIS
        ) PURE;


    STDMETHOD(CreateNoValue)(
        THIS_
        _Out_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateErrorObject)(
        THIS_
        _In_ HRESULT hrError,
        _In_opt_ PCWSTR pwszMessage,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateTypedObject)(
        THIS_
        _In_opt_ IDebugHostContext* context,
        _In_ Location objectLocation,
        _In_ IDebugHostType* objectType,
        _COM_Errorptr_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateTypedObjectReference)(
        THIS_
        _In_opt_ IDebugHostContext* context,
        _In_ Location objectLocation,
        _In_ IDebugHostType* objectType,
        _COM_Errorptr_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateSyntheticObject)(
        THIS_
        _In_opt_ IDebugHostContext* context,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateDataModelObject)(
        THIS_
        _In_ IDataModelConcept* dataModel,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateIntrinsicObject)(
        THIS_
        _In_ ModelObjectKind objectKind,
        _In_ VARIANT* intrinsicData,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateTypedIntrinsicObject)(
        THIS_
        _In_ VARIANT* intrinsicData,
        _In_ IDebugHostType* type,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    STDMETHOD(GetModelForTypeSignature)(
        THIS_
        _In_ IDebugHostTypeSignature* typeSignature,
        _Out_ IModelObject** dataModel
        ) PURE;

    STDMETHOD(GetModelForType)(
        THIS_
        _In_ IDebugHostType* type,
        _COM_Outptr_ IModelObject** dataModel,
        _COM_Outptr_opt_ IDebugHostTypeSignature** typeSignature,
        _COM_Outptr_opt_ IDebugHostSymbolEnumerator** wildcardMatches
        ) PURE;

    STDMETHOD(RegisterModelForTypeSignature)(
        THIS_
        _In_ IDebugHostTypeSignature* typeSignature,
        _In_ IModelObject* dataModel
        ) PURE;

    STDMETHOD(UnregisterModelForTypeSignature)(
        THIS_
        _In_ IModelObject* dataModel,
        _In_opt_ IDebugHostTypeSignature* typeSignature
        ) PURE;

    STDMETHOD(RegisterExtensionForTypeSignature)(
        THIS_
        _In_ IDebugHostTypeSignature* typeSignature,
        _In_ IModelObject* dataModel
        ) PURE;

    STDMETHOD(UnregisterExtensionForTypeSignature)(
        THIS_
        _In_ IModelObject* dataModel,
        _In_opt_ IDebugHostTypeSignature* typeSignature
        ) PURE;

    STDMETHOD(CreateMetadataStore)(
        THIS_
        _In_opt_ IKeyStore* parentStore,
        _COM_Outptr_ IKeyStore** metadataStore
        ) PURE;

    STDMETHOD(GetRootNamespace)(
        THIS_
        _COM_Outptr_ IModelObject** rootNamespace
        ) PURE;

    STDMETHOD(RegisterNamedModel)(
        THIS_
        _In_ PCWSTR modelName,
        _In_ IModelObject *modeObject
        ) PURE;

    STDMETHOD(UnregisterNamedModel)(
        THIS_
        _In_ PCWSTR modelName
        ) PURE;

    STDMETHOD(AcquireNamedModel)(
        THIS_
        _In_ PCWSTR modelName,
        _COM_Outptr_ IModelObject **modelObject
        ) PURE;

    //*************************************************
    // IDataModelManager2
    //

    // AcquireSubNamespace():
    //
    // A convenience method for acquiring (and registering if necessary) a sub-namespace on an object.
    //
    // modelName
    //     the name of the model which is being extended with a namespace (e.g.: "Debugger.Models.Process")
    //
    // subNamespaceModelName
    //     the name of the model which is being added (e.g.: "Debugger.Models.Process.Io")
    //
    // accessName
    //     the name used to access the namespace from the parent object (e.g.: "Io")
    //
    // metadata
    //     the metadata store used on the accessor for the namespace (e.g.: the help on "Io" if it is newly created)
    //
    // namespaceModelObject
    //     the namespace model returned is placed here
    //
    STDMETHOD(AcquireSubNamespace)(
        THIS_
        _In_ PCWSTR modelName,
        _In_ PCWSTR subNamespaceModelName,
        _In_ PCWSTR accessName,
        _In_opt_ IKeyStore *metadata,
        _COM_Outptr_ IModelObject **namespaceModelObject
        ) PURE;

    // CreateTypedIntrinsicObjectEx():
    //
    // A version of CreateTypedIntrinsicObject which allows for the passing of an explicit context.  Such is only
    // useful for intrinsics which represent addresses in the target (such as pointers).
    //
    STDMETHOD(CreateTypedIntrinsicObjectEx)(
        THIS_
        _In_opt_ IDebugHostContext* context,
        _In_ VARIANT* intrinsicData,
        _In_ IDebugHostType* type,
        _COM_Outptr_ IModelObject** object
        ) PURE;

};

//
// IDataModelManager3:
//
// The third version of the interface for the data model manager. This is the interface by which new objects are created,
// intrinsic values are boxed and unboxed, and models are registered for types.
//
// This interface is never directly implemented by a client.
//
#undef INTERFACE
#define INTERFACE IDataModelManager3
DECLARE_INTERFACE_(IDataModelManager3, IDataModelManager2)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelManager

    STDMETHOD(Close)(
        THIS
        ) PURE;


    STDMETHOD(CreateNoValue)(
        THIS_
        _Out_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateErrorObject)(
        THIS_
        _In_ HRESULT hrError,
        _In_opt_ PCWSTR pwszMessage,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateTypedObject)(
        THIS_
        _In_opt_ IDebugHostContext* context,
        _In_ Location objectLocation,
        _In_ IDebugHostType* objectType,
        _COM_Errorptr_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateTypedObjectReference)(
        THIS_
        _In_opt_ IDebugHostContext* context,
        _In_ Location objectLocation,
        _In_ IDebugHostType* objectType,
        _COM_Errorptr_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateSyntheticObject)(
        THIS_
        _In_opt_ IDebugHostContext* context,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateDataModelObject)(
        THIS_
        _In_ IDataModelConcept* dataModel,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateIntrinsicObject)(
        THIS_
        _In_ ModelObjectKind objectKind,
        _In_ VARIANT* intrinsicData,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateTypedIntrinsicObject)(
        THIS_
        _In_ VARIANT* intrinsicData,
        _In_ IDebugHostType* type,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    STDMETHOD(GetModelForTypeSignature)(
        THIS_
        _In_ IDebugHostTypeSignature* typeSignature,
        _Out_ IModelObject** dataModel
        ) PURE;

    STDMETHOD(GetModelForType)(
        THIS_
        _In_ IDebugHostType* type,
        _COM_Outptr_ IModelObject** dataModel,
        _COM_Outptr_opt_ IDebugHostTypeSignature** typeSignature,
        _COM_Outptr_opt_ IDebugHostSymbolEnumerator** wildcardMatches
        ) PURE;

    STDMETHOD(RegisterModelForTypeSignature)(
        THIS_
        _In_ IDebugHostTypeSignature* typeSignature,
        _In_ IModelObject* dataModel
        ) PURE;

    STDMETHOD(UnregisterModelForTypeSignature)(
        THIS_
        _In_ IModelObject* dataModel,
        _In_opt_ IDebugHostTypeSignature* typeSignature
        ) PURE;

    STDMETHOD(RegisterExtensionForTypeSignature)(
        THIS_
        _In_ IDebugHostTypeSignature* typeSignature,
        _In_ IModelObject* dataModel
        ) PURE;

    STDMETHOD(UnregisterExtensionForTypeSignature)(
        THIS_
        _In_ IModelObject* dataModel,
        _In_opt_ IDebugHostTypeSignature* typeSignature
        ) PURE;

    STDMETHOD(CreateMetadataStore)(
        THIS_
        _In_opt_ IKeyStore* parentStore,
        _COM_Outptr_ IKeyStore** metadataStore
        ) PURE;

    STDMETHOD(GetRootNamespace)(
        THIS_
        _COM_Outptr_ IModelObject** rootNamespace
        ) PURE;

    STDMETHOD(RegisterNamedModel)(
        THIS_
        _In_ PCWSTR modelName,
        _In_ IModelObject *modeObject
        ) PURE;

    STDMETHOD(UnregisterNamedModel)(
        THIS_
        _In_ PCWSTR modelName
        ) PURE;

    STDMETHOD(AcquireNamedModel)(
        THIS_
        _In_ PCWSTR modelName,
        _COM_Outptr_ IModelObject **modelObject
        ) PURE;

    //*************************************************
    // IDataModelManager2
    //

    // AcquireSubNamespace():
    //
    // A convenience method for acquiring (and registering if necessary) a sub-namespace on an object.
    //
    // modelName
    //     the name of the model which is being extended with a namespace (e.g.: "Debugger.Models.Process")
    //
    // subNamespaceModelName
    //     the name of the model which is being added (e.g.: "Debugger.Models.Process.Io")
    //
    // accessName
    //     the name used to access the namespace from the parent object (e.g.: "Io")
    //
    // metadata
    //     the metadata store used on the accessor for the namespace (e.g.: the help on "Io" if it is newly created)
    //
    // namespaceModelObject
    //     the namespace model returned is placed here
    //
    STDMETHOD(AcquireSubNamespace)(
        THIS_
        _In_ PCWSTR modelName,
        _In_ PCWSTR subNamespaceModelName,
        _In_ PCWSTR accessName,
        _In_opt_ IKeyStore *metadata,
        _COM_Outptr_ IModelObject **namespaceModelObject
        ) PURE;

    // CreateTypedIntrinsicObjectEx():
    //
    // A version of CreateTypedIntrinsicObject which allows for the passing of an explicit context.  Such is only
    // useful for intrinsics which represent addresses in the target (such as pointers).
    //
    STDMETHOD(CreateTypedIntrinsicObjectEx)(
        THIS_
        _In_opt_ IDebugHostContext* context,
        _In_ VARIANT* intrinsicData,
        _In_ IDebugHostType* type,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    //*************************************************
    // IDataModelManager3
    //

    //
    // A convenience method for acquiring (and registering if necessary) a filtered sub-namespace on an object.
    //
    // modelName
    //     the name of the model which is being extended with a namespace (e.g.: "Debugger.Models.Process")
    //
    // subNamespaceModelName
    //     the name of the model which is being added (e.g.: "Debugger.Models.Process.Io")
    //
    // accessName
    //     the name used to access the namespace from the parent object (e.g.: "Io")
    //
    // filter
    //     the filter method to evaluate the context object in order to determine if the namespace property will be applied to the context object
    //
    // metadata
    //     the metadata store used on the accessor for the namespace (e.g.: the help on "Io" if it is newly created)
    //
    // namespaceModelObject
    //     the namespace model returned is placed here
    //
    // token
    //     the token returned is placed here.
    //
    STDMETHOD(AcquireFilteredSubNamespace)(
        THIS_
        _In_ PCWSTR modelName,
        _In_ PCWSTR subNamespaceModelName,
        _In_ PCWSTR accessName,
        _In_opt_ IKeyStore *metadata,
        _In_ IModelMethod *filter,
        _COM_Outptr_ IModelObject **namespaceModelObject,
        _COM_Outptr_result_maybenull_ IFilteredNamespacePropertyToken **token
        ) PURE;

    // EnumerateNamedModels():
    //
    // This returns an enumerator which will enumerate all registered named models and their associated name.
    //
    STDMETHOD(EnumerateNamedModels)(
        THIS_
        _COM_Outptr_ INamedModelsEnumerator** ppEnumerator
        ) PURE;
};

//
// IDataModelManager4:
//
// It adds debug model reporting functionality.
//
// This interface is never directly implemented by a client.
//
#undef INTERFACE
#define INTERFACE IDataModelManager4
DECLARE_INTERFACE_(IDataModelManager4, IDataModelManager3)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelManager

    STDMETHOD(Close)(
        THIS_
        ) PURE;


    STDMETHOD(CreateNoValue)(
        THIS_
        _Out_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateErrorObject)(
        THIS_
        _In_ HRESULT hrError,
        _In_opt_ PCWSTR pwszMessage,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateTypedObject)(
        THIS_
        _In_opt_ IDebugHostContext* context,
        _In_ Location objectLocation,
        _In_ IDebugHostType* objectType,
        _COM_Errorptr_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateTypedObjectReference)(
        THIS_
        _In_opt_ IDebugHostContext* context,
        _In_ Location objectLocation,
        _In_ IDebugHostType* objectType,
        _COM_Errorptr_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateSyntheticObject)(
        THIS_
        _In_opt_ IDebugHostContext* context,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateDataModelObject)(
        THIS_
        _In_ IDataModelConcept* dataModel,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateIntrinsicObject)(
        THIS_
        _In_ ModelObjectKind objectKind,
        _In_ VARIANT* intrinsicData,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    STDMETHOD(CreateTypedIntrinsicObject)(
        THIS_
        _In_ VARIANT* intrinsicData,
        _In_ IDebugHostType* type,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    STDMETHOD(GetModelForTypeSignature)(
        THIS_
        _In_ IDebugHostTypeSignature* typeSignature,
        _Out_ IModelObject** dataModel
        ) PURE;

    STDMETHOD(GetModelForType)(
        THIS_
        _In_ IDebugHostType* type,
        _COM_Outptr_ IModelObject** dataModel,
        _COM_Outptr_opt_ IDebugHostTypeSignature** typeSignature,
        _COM_Outptr_opt_ IDebugHostSymbolEnumerator** wildcardMatches
        ) PURE;

    STDMETHOD(RegisterModelForTypeSignature)(
        THIS_
        _In_ IDebugHostTypeSignature* typeSignature,
        _In_ IModelObject* dataModel
        ) PURE;

    STDMETHOD(UnregisterModelForTypeSignature)(
        THIS_
        _In_ IModelObject* dataModel,
        _In_opt_ IDebugHostTypeSignature* typeSignature
        ) PURE;

    STDMETHOD(RegisterExtensionForTypeSignature)(
        THIS_
        _In_ IDebugHostTypeSignature* typeSignature,
        _In_ IModelObject* dataModel
        ) PURE;

    STDMETHOD(UnregisterExtensionForTypeSignature)(
        THIS_
        _In_ IModelObject* dataModel,
        _In_opt_ IDebugHostTypeSignature* typeSignature
        ) PURE;

    STDMETHOD(CreateMetadataStore)(
        THIS_
        _In_opt_ IKeyStore* parentStore,
        _COM_Outptr_ IKeyStore** metadataStore
        ) PURE;

    STDMETHOD(GetRootNamespace)(
        THIS_
        _COM_Outptr_ IModelObject** rootNamespace
        ) PURE;

    STDMETHOD(RegisterNamedModel)(
        THIS_
        _In_ PCWSTR modelName,
        _In_ IModelObject *modeObject
        ) PURE;

    STDMETHOD(UnregisterNamedModel)(
        THIS_
        _In_ PCWSTR modelName
        ) PURE;

    STDMETHOD(AcquireNamedModel)(
        THIS_
        _In_ PCWSTR modelName,
        _COM_Outptr_ IModelObject **modelObject
        ) PURE;

    //*************************************************
    // IDataModelManager2
    //

    // AcquireSubNamespace():
    //
    // A convenience method for acquiring (and registering if necessary) a sub-namespace on an object.
    //
    // modelName
    //     the name of the model which is being extended with a namespace (e.g.: "Debugger.Models.Process")
    //
    // subNamespaceModelName
    //     the name of the model which is being added (e.g.: "Debugger.Models.Process.Io")
    //
    // accessName
    //     the name used to access the namespace from the parent object (e.g.: "Io")
    //
    // metadata
    //     the metadata store used on the accessor for the namespace (e.g.: the help on "Io" if it is newly created)
    //
    // namespaceModelObject
    //     the namespace model returned is placed here
    //
    STDMETHOD(AcquireSubNamespace)(
        THIS_
        _In_ PCWSTR modelName,
        _In_ PCWSTR subNamespaceModelName,
        _In_ PCWSTR accessName,
        _In_opt_ IKeyStore *metadata,
        _COM_Outptr_ IModelObject **namespaceModelObject
        ) PURE;

    // CreateTypedIntrinsicObjectEx():
    //
    // A version of CreateTypedIntrinsicObject which allows for the passing of an explicit context.  Such is only
    // useful for intrinsics which represent addresses in the target (such as pointers).
    //
    STDMETHOD(CreateTypedIntrinsicObjectEx)(
        THIS_
        _In_opt_ IDebugHostContext* context,
        _In_ VARIANT* intrinsicData,
        _In_ IDebugHostType* type,
        _COM_Outptr_ IModelObject** object
        ) PURE;

    //*************************************************
    // IDataModelManager3
    //

    //
    // A convenience method for acquiring (and registering if necessary) a filtered sub-namespace on an object.
    //
    // modelName
    //     the name of the model which is being extended with a namespace (e.g.: "Debugger.Models.Process")
    //
    // subNamespaceModelName
    //     the name of the model which is being added (e.g.: "Debugger.Models.Process.Io")
    //
    // accessName
    //     the name used to access the namespace from the parent object (e.g.: "Io")
    //
    // filter
    //     the filter method to evaluate the context object in order to determine if the namespace property will be applied to the context object
    //
    // metadata
    //     the metadata store used on the accessor for the namespace (e.g.: the help on "Io" if it is newly created)
    //
    // namespaceModelObject
    //     the namespace model returned is placed here
    //
    // token
    //     the token returned is placed here.
    //
    STDMETHOD(AcquireFilteredSubNamespace)(
        THIS_
        _In_ PCWSTR modelName,
        _In_ PCWSTR subNamespaceModelName,
        _In_ PCWSTR accessName,
        _In_opt_ IKeyStore *metadata,
        _In_ IModelMethod *filter,
        _COM_Outptr_ IModelObject **namespaceModelObject,
        _COM_Outptr_result_maybenull_ IFilteredNamespacePropertyToken **token
        ) PURE;

    // EnumerateNamedModels():
    //
    // This returns an enumerator which will enumerate all registered named models and their associated name.
    //
    STDMETHOD(EnumerateNamedModels)(
        THIS_
        _COM_Outptr_ INamedModelsEnumerator** ppEnumerator
        ) PURE;

    //*************************************************
    // IDataModelManager4
    //

    // CreateSyntheticObjectFromKeyStore():
    //
    // Creates a synthetic object from an existing key store (key/value/metadata tuples).
    //
    STDMETHOD(CreateSyntheticObjectFromKeyStore)(
        THIS_
        _In_opt_ IDebugHostContext* context,
        _In_ IKeyStore* parentStore,
        _COM_Outptr_ IModelObject** object
        ) PURE;
};

//
// IModelKeyReference:
//
// Represents a reference to a key which can be resolved (dereferenced) to get the underlying key.
//
// This interface is never directly implemented by a client.
//
#undef INTERFACE
#define INTERFACE IModelKeyReference
DECLARE_INTERFACE_(IModelKeyReference, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IModelKeyReference:

    // GetKeyName():
    //
    // Gets the name of the key this reference refers to.
    //
    STDMETHOD(GetKeyName)(
        THIS_
        _Out_ BSTR* keyName
        ) PURE;

    // GetOriginalObject():
    //
    // Gets the object on which the original "GetKeyReference" which produced this
    // key reference was called.
    //
    // If a key reference is resolved to a property accessor with GetKey(), the original
    // object should be passed as context.  The IModelPropertyAccessor object which
    // is acquired from the resolution will automatically perform the necessary adjustment
    // thunk between the return value of this method and the return value of
    // GetContextObject().
    //
    STDMETHOD(GetOriginalObject)(
        THIS_
        _COM_Outptr_ IModelObject** originalObject
        ) PURE;

    // GetContextObject():
    //
    // Gets the (potentially adjusted) context object for the key.  If the key was found on
    // a parent model that had a context adjustor, this will be different from the value returned
    // in GetOriginalObject; otherwise -- it will be identical.
    //
    // This object is not typically needed.
    //
    STDMETHOD(GetContextObject)(
        THIS_
        _COM_Outptr_ IModelObject** containingObject
        ) PURE;

    //*************************************************
    // Accessors:
    //

    //
    // These are the **ONLY** valid way to resolve a key reference.  The particular parent model
    // on which the key was found is not exposed.  There is no way for a caller to fetch
    // the particular key by using an IModelObject->GetKey(this->GetKeyName()) for any
    // given IModelObject.
    //

    // GetKey():
    //
    // Resolves (dereferences) the reference and returns the underlying value as if a call to the
    // "GetKey" method were made on the IModelObject from which this key reference was derived.
    //
    // Note that this method will not resolve underlying properties.  If the value of the key is
    // a property accessor, it will return it.
    //
    STDMETHOD(GetKey)(
        THIS_
        _COM_Errorptr_opt_ IModelObject** object,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    // GetKeyValue():
    //
    // Resolves (dereferences) the reference and returns the underlying value as if a call to the
    // "GetKeyValue" method were made on the IModelObject from which this key reference was derived.
    //
    // Note that this method will resolve underlying properties.  If the value of the key is
    // a property accessor, this API will fetch the value underlying the property and return it.
    // It will not return the IModelPropertyAccessor interface for the property.
    //
    STDMETHOD(GetKeyValue)(
        THIS_
        _COM_Errorptr_opt_ IModelObject** object,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    // SetKey():
    //
    // Resolves (dereferences) the reference and sets the underlying key.  Note that this does not
    // behave identically to a "SetKey" call on the IModelObject from which this key reference was
    // derived.
    //
    // This will always set the value of the PARTICULAR key to which the reference refers.
    //
    STDMETHOD(SetKey)(
        THIS_
        _In_opt_ IModelObject* object,
        _In_opt_ IKeyStore* metadata
        ) PURE;

    // SetKeyValue():
    //
    // Resolves (dereferences) the reference and sets the underlying key.
    //
    // Note that this method will resolve underlying properties.  If the value of the key is
    // a property accessor, this API will call the set method on the underlying property.
    //
    STDMETHOD(SetKeyValue)(
        THIS_
        _In_ IModelObject* object
        ) PURE;
};

//
// IModelPropertyAccessor:
//
// Represents a read-only or read-write property on an object.  A client which only wishes to fetch the value
// of a property would normally call a "GetKeyValue" method on an object and not deal directly with this interface.
//
// Extensions which implement properties would implement this interface one or more times for the properties
// which it provides.
//
#undef INTERFACE
#define INTERFACE IModelPropertyAccessor
DECLARE_INTERFACE_(IModelPropertyAccessor, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IModelPropertyAccessor:

    // GetValue():
    //
    // Gets the underlying value of the property.  The name of the key and the original context object
    // passed to the "GetKey" method that returned this property accessor must be passed to this
    // call.
    //
    STDMETHOD(GetValue)(
        THIS_
        _In_ PCWSTR key,
        _In_opt_ IModelObject* contextObject,
        _COM_Outptr_ IModelObject** value
        ) PURE;

    // SetValue():
    //
    // Sets the underlying value of the property.  The name of the key and the original context object
    // passed to the "GetKey" method that returned this property accessor must be passed to this
    // call.
    //
    // Note that some properties are read-only.  In such a case, this method will fail.
    //
    STDMETHOD(SetValue)(
        THIS_
        _In_ PCWSTR key,
        _In_opt_ IModelObject* contextObject,
        _In_ IModelObject* value
        ) PURE;
};

//
// IModelMethod:
//
// Represents a method which can be called.
//
// Extensions which implement methods would implement this interface one or more times for the methods
// which it provides.
//
#undef INTERFACE
#define INTERFACE IModelMethod
DECLARE_INTERFACE_(IModelMethod, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IModelMethod:


    // Call():
    //
    // Call the method.  The arguments to the method must be packed into a linear
    // array.  A singular result (or error) is returned.  Metadata about the result
    // can also optionally be returned.
    //
    STDMETHOD(Call)(
        THIS_
        _In_opt_ IModelObject *pContextObject,
        _In_ ULONG64 argCount,
        _In_reads_(argCount) IModelObject **ppArguments,
        _COM_Errorptr_ IModelObject **ppResult,
        _COM_Outptr_opt_result_maybenull_ IKeyStore **ppMetadata
        ) PURE;
};

//
// IKeyEnumerator:
//
// An interface which enumerates the model based keys on an object (and their values and associated
// metadata).  A key enumerator can be acquired through the EnumerateKeys, EnumerateKeyValues, and
// EnumerateKeyReferences methods on an IModelObject.
//
#undef INTERFACE
#define INTERFACE IKeyEnumerator
DECLARE_INTERFACE_(IKeyEnumerator, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IKeyEnumerator:

    // Reset():
    //
    // Resets the enumerator to its initial state.  A subsequent GetNext call will return
    // the first key in enumerator order.
    //
    STDMETHOD(Reset)(
        THIS
        ) PURE;

    // GetNext():
    //
    // Moves the iterator forward and fetches the name of the next key and, optionally, its value (or a
    // reference to it) and associated metadata.
    //
    // Note that depending on how this enumerator was acquired, the object returned in the value field
    // may be the value associated with the key (EnumerateKeys), the resolved value of any property that
    // the key referes to (EnumerateKeyValues), or a reference to the key (EnumerateKeyReferences).
    //
    // If there was an error in resolving the value of the key (for EnumerateKeyValues, for instance),
    // the method may return an error *AND* fill value with an error object.
    //
    // When the enumerator hits the end of the sequence, E_BOUNDS will be returned.
    //
    STDMETHOD(GetNext)(
        _Out_ BSTR* key,
        _COM_Errorptr_opt_ IModelObject** value,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

};

//
// IRawEnumerator:
//
// An interface which enumerates the raw children (e.g.: base classes, fields, etc...) of an object
// (and their values and associated metadata).  A raw enumerator can be acquired through the
// EnumerateRawValues or EnumerateRawReferences methods on IModelObject.
//
#undef INTERFACE
#define INTERFACE IRawEnumerator
DECLARE_INTERFACE_(IRawEnumerator, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IKeyEnumerator:

    // Reset():
    //
    // Resets the enumerator to its initial state.  A subsequent GetNext call will return
    // the first raw element (native field, base class, etc...) in enumerator order.
    //
    STDMETHOD(Reset)(
        THIS
        ) PURE;

    // GetNext():
    //
    // Moves the iterator forward and fetches the name of the raw element and, optionally, its value (or a
    // reference to it) and what kind of element it is.
    //
    // Note that depending on how this enumerator was acquired, the object returned in the value field
    // may be the value of the raw element (EnumerateRawValues) or a reference to the raw element
    // (EnumerateRawReferences).
    //
    // If there was an error in reading the value of the raw element (for EnumerateRawValues, for instance),
    // the method may return an error *AND* fill value with an error object.
    //
    // When the enumerator hits the end of the sequence, E_BOUNDS will be returned.
    //
    STDMETHOD(GetNext)(
        _Out_opt_ BSTR* name,
        _Out_opt_ SymbolKind *kind,
        _COM_Errorptr_opt_ IModelObject** value
        ) PURE;

};

//
// INamedModelsEnumerator:
//
// An interface which enumerates registered named models
//
#undef INTERFACE
#define INTERFACE INamedModelsEnumerator
DECLARE_INTERFACE_(INamedModelsEnumerator, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // INamedModelsEnumerator:

    // Reset():
    //
    // Resets the enumerator.
    //
    STDMETHOD(Reset)(
        THIS
        ) PURE;

    // GetNext():
    //
    // Moves the iterator forward and fetches the name of the next registered model name and the model
    //
    STDMETHOD(GetNext)(
        THIS_
        _Out_ BSTR* pModelName,
        _COM_Outptr_ IModelObject** ppModel
        ) PURE;
};

//
// IDataModelConcept:
//
// Any object which represents a data model which is registered under a name or
// is registered for a particular type signature must implement this concept and add it to the data model
// object via IModelObject::SetConcept.
//
// Clients which create data models implement this interface.  It is most frequently
// consumed by the data model manager itself.
//
#undef INTERFACE
#define INTERFACE IDataModelConcept
DECLARE_INTERFACE_(IDataModelConcept, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelConcept:

    // InitializeObject():
    //
    // If a particular data model is attached to a type signature
    // (via IDataModelManager::RegisterModelForTypeSignature) before an instance of a type matching
    // that signature is created, this method will be called on the model.  The method will be passed
    // the instance object which is being created (modelObject), the type signature which matched
    // the specific type of the instance object (matchingTypeSignature), and an enumerator to any wildcard
    // matches between the specific type and the type signature.
    //
    // Note that a data model implementation **MUST NOT** rely on this method being called.  A data model
    // may be attached after instances of a particular type already exist.  This method is most frequently
    // used for caching purposes.
    //
    // A client of the model never calls this API directly.  It is called by the model itself.  An
    // implementor may choose to do nothing in the method; however -- any such "do nothing" implementation
    // must still succeed via an S_OK return.  A failure returned from this method will prevent object
    // construction.
    //
    STDMETHOD(InitializeObject)(
        THIS_
        _In_ IModelObject* modelObject,
        _In_opt_ IDebugHostTypeSignature* matchingTypeSignature,
        _In_opt_ IDebugHostSymbolEnumerator* wildcardMatches
        ) PURE;

    // GetName():
    //
    // Returns the name of the data model.  If the data model is registered under a default name
    // (via IDataModelManager::RegisterNamedModel), it is expected that the returned name is the
    // registered default name.  Note that a data model may be registered under multiple names.
    // It is also perfectly legitimate for a data model to be completely unnamed.  In such cases,
    // the GetName method may return E_NOTIMPL.
    //
    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* modelName
        ) PURE;

};

//
// IStringDisplayableConcept:
//
// Any object (or data model) which has a string conversion suitable for display implements this concept.
// Clients should not rely on the form of this string conversion for programmatic purposes.  It is intended
// for display purposes only.
//
#undef INTERFACE
#define INTERFACE IStringDisplayableConcept
DECLARE_INTERFACE_(IStringDisplayableConcept, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IStringDisplayableConcept:

    // ToDisplayString():
    //
    // Called in order to convert an instance of an object to a string suitable for display.
    // The "contextObject" argument refers to the object being converted.  If there is metadata which
    // governs the string conversion (e.g.: choosing which radix to convert an ordinal in), the associated
    // metadata store is passed in the "metadata" argument.
    //
    STDMETHOD(ToDisplayString)(
        THIS_
        _In_ IModelObject* contextObject,
        _In_opt_ IKeyStore* metadata,
        _Out_ BSTR* displayString
        ) PURE;
};

//
// ICodeAddressConcept:
//
// ICodeAddressConcept Description
#undef INTERFACE
#define INTERFACE ICodeAddressConcept
DECLARE_INTERFACE_(ICodeAddressConcept, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // Name:

    // GetContainingFunctionSymbol():
    //
    // GetContainingFunctionSymbol Description
    STDMETHOD(GetContainingSymbol)(
        THIS_
        _In_ IModelObject* pContextObject,
        _Out_ IDebugHostSymbol **ppSymbol
        ) PURE;
};

//
// IModelIterator:
//
// This interface enumerates the iterable elements of a collection.  An object which is iterable
// via IIterableConcept must return an implementation of IModelIterator from its GetIterator method.
//
#undef INTERFACE
#define INTERFACE IModelIterator
DECLARE_INTERFACE_(IModelIterator, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IModelIterator:

    // Reset():
    //
    // Resets the enumerator to its initial state.  A subsequent GetNext call will return
    // the first iterated element in iteration order.
    //
    // Note that it is legitimate for a particular iterator to E_NOTIMPL this particular
    // call.  In such circumstances, the iterator is forward only and cannot be reset to its
    // initial state.  While this is legitimate, it is highly recommended that any iterator which
    // can support the reset capability.
    //
    STDMETHOD(Reset)(
        THIS
        ) PURE;

    // GetNext():
    //
    // Moves the iterator forward and fetches the next iterated element and, optionally, the default
    // indexer for that element.  Note that any iterable which returns a non-zero value from
    // IIterableConcept::GetDefaultIndexDimensionality *MUST* return that dimensionality of indexer
    // from any call to GetNext.  A client of the GetNext method may choose to pass 0/nullptr and not
    // retrieve the indexer or choose to pass the dimensionality and a buffer of that size to retrieve
    // the indexer.  It is illegal to request or pass back only part of an indexer via a non-zero "dimensions"
    // argument which is less than the default index dimensionality returned from
    // IIterableConcept::GetDefaultIndexDimensionality.
    //
    // If the iterator moved forward successfully but there was an error in reading the value of
    // the iterated element, the method may return an error *AND* fill "object" with an error object.
    //
    // When the enumerator hits the end of the sequence, E_BOUNDS will be returned.
    //
    STDMETHOD(GetNext)(
        _COM_Errorptr_ IModelObject** object,
        _In_ ULONG64 dimensions,
        _Out_writes_opt_(dimensions) IModelObject** indexers,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;
};

// WrappedObjectPreference:
//
// Indicates a preference for how the wrapper and the wrapped object should be treated.
//
enum WrappedObjectPreference
{
    // Indicates that the wrapped object should be used for name resolution and not interpreted to be a
    // generalized proxy for the object.  In essence, things like "." and "->" should work in an expression
    // evaluator; however, other operations should not.
    WrappedObjectNameResolution,

    // Indicates that the wrapper should be considered a general proxy for the wrapped object.
    WrappedObjectGeneralProxy
};

// IObjectWrapperConcept:
//
// An object which is a wrapper for another object (e.g.: a smart pointer like std::unique_ptr)
// can implement this concept to indicate such.
//
#undef INTERFACE
#define INTERFACE IObjectWrapperConcept
DECLARE_INTERFACE_(IObjectWrapperConcept, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IObjectWrapperConcept:

    // GetWrappedObject():
    //
    // Gets the object that the context object wraps (e.g.: the underlying pointer which a smart pointer object holds)
    //
    STDMETHOD(GetWrappedObject)(
        THIS_
        _In_ IModelObject *pContextObject,
        _COM_Outptr_ IModelObject **wrappedObject,
        _Out_ WrappedObjectPreference *pUsagePreference
        ) PURE;
};

//
// IIterableConcept:
//
// Any object (or data model) which is a container which can be iterated implements this concept.
//
#undef INTERFACE
#define INTERFACE IIterableConcept
DECLARE_INTERFACE_(IIterableConcept, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IIterableConcept:

    // GetDefaultIndexDimensionality():
    //
    // If the iterable object also supports indexing via supporting IIndexableConcept, the iterator may optionally
    // return a "default index" for each element it produces.  The dimensionality of that "default index" is returned
    // from this method.
    //
    // An implementation of this method may return 0 / S_OK indicating that there is no default index returned
    // from the iterator or it may return non-zero / S_OK indicating that there is a default index returned from
    // each GetNext of the iterator.
    //
    // Note that an implementation which returns non-zero for dimensionality here is promising:
    //
    //     - It supports IIndexableConcept
    //
    //     - The GetNext method of the IModelIterator returned from the GetIterator method here will return a default
    //       index of this dimensionality for each element that is UNIQUE
    //
    //     - Passing the default index from IModelIterator::GetNext to IIndexableConcept::GetAt will return the same
    //       value
    //
    STDMETHOD(GetDefaultIndexDimensionality)(
        THIS_
        _In_ IModelObject* contextObject,
        _Out_ ULONG64* dimensionality
        ) PURE;

    // GetIterator():
    //
    // Returns an iterator instance for this iterable.  The instance of the object being iterated is passed in
    // the "contextObject" argument.
    //
    STDMETHOD(GetIterator)(
        THIS_
        _In_ IModelObject* contextObject,
        _Out_ IModelIterator** iterator
        ) PURE;

};

//
// IIndexableConcept:
//
// Any object which is a container that supports random access retrieval of elements from given N-dimensional
// indexers implements this concept.
//
// It is legal for an object to be indexable (via support of IIndexableConcept) and not iterable (via lack of support
// for IIterableConcept).
//
#undef INTERFACE
#define INTERFACE IIndexableConcept
DECLARE_INTERFACE_(IIndexableConcept, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IIndexableConcept:

    // GetDimensionality():
    //
    // Returns the dimensionality of the indexer.
    //
    // Note that if the object in question is iterable as well as indexable and the object supports a
    // default indexer (as inquired through IIterableConcept::GetDefaultIndexDimensionality), the
    // dimensionality returned from the iterator and this method must agree.
    //
    STDMETHOD(GetDimensionality)(
        THIS_
        _In_ IModelObject* contextObject,
        _Out_ ULONG64* dimensionality
        ) PURE;

    // GetAt():
    //
    // Returns the value of the element at a particular N-dimensional index.  An indexer of N-dimensions
    // where N is the value returned from GetDimensionality **MUST** be supported.
    //
    // Note that a given object may be indexable in different domains by different types (e.g.: indexable
    // via both ordinals and strings).
    //
    // If the index is out of range (or could not be accessed), the method will return a failure; however,
    // in such cases, the output object may still be set to an error object.
    //
    STDMETHOD(GetAt)(
        THIS_
        _In_ IModelObject* contextObject,
        _In_ ULONG64 indexerCount,
        _In_reads_(indexerCount) IModelObject** indexers,
        _COM_Errorptr_ IModelObject** object,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    // SetAt():
    //
    // Attempts to set the value of the element at a particular N-dimensional index.  An indexer of N-dimensions
    // where N is the value returned from GetDimensionality **MUST** be supported.
    //
    // Note that a given object may be indexable in different domains by different types (e.g.: indexable
    // via both ordinals and strings).
    //
    // If the index is out of range (or could not be accessed), the method will return a failure; however,
    // in such cases, the output object may still be set to an error object.
    //
    // Note that some indexers are read-only.  This method may fail on such indexers (with E_NOTIMPL).
    //
    STDMETHOD(SetAt)(
        THIS_
        _In_ IModelObject* contextObject,
        _In_ ULONG64 indexerCount,
        _In_reads_(indexerCount) IModelObject** indexers,
        _In_ IModelObject *value
        ) PURE;
};

//
// IPreferredRuntimeTypeConcept:
//
// This interface provides a facility to use custom logic when casting an object to it's runtime type. For most types this is not
// needed as in most instances the runtime type can be determined by the debugger automatically. In some cases this is not
// possible and this interface can be used in order to provide the correct runtime type.
//
#undef INTERFACE
#define INTERFACE IPreferredRuntimeTypeConcept
DECLARE_INTERFACE_(IPreferredRuntimeTypeConcept, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IPreferredRuntimeTypeConcept:

    // CastToPreferredRuntimeType()
    //
    // Casts the object to it's preferred runtime type.
    //
    // If the cast is successful the method will return success.
    // Otherwise a failure is returned.
    //
    // Note that the error E_NOT_SET is considered special by this method.
    // An implementation of this method which returns E_NOT_SET is indicating
    // to the data model that it does not wish to override the default
    // (type system based) conversion to a runtime type.
    //
    STDMETHOD(CastToPreferredRuntimeType)(
        THIS_
        _In_ IModelObject* contextObject,
        _COM_Errorptr_ IModelObject** object
        ) PURE;
};

//
// IDebugHost:
//
// The core interface which represents the underlying debugger which is hosting the data model.
// The host is required to support this interface.  The various other IDebugHost* interfaces
// can be queried once an extension has access to the debug host.
//
#undef INTERFACE
#define INTERFACE IDebugHost
DECLARE_INTERFACE_(IDebugHost, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHost:

    // GetHostDefinedInterface():
    //
    // Returns an object (interface cast to IUnknown) which only has meaning to a specific host.
    // The DbgEng host of the model might return an IDebugClient (cast to IUnknown) as an example.
    //
    // A host may E_NOTIMPL this if it does not wish to provide any access to its implementation
    // specific interfaces.
    //
    STDMETHOD(GetHostDefinedInterface)(
        THIS_
        _COM_Outptr_ IUnknown** hostUnk
        ) PURE;

    // GetCurrentContext():
    //
    // Returns a context object which represents the current "state" of the debugger.  This is the context
    // in which all expression evaluations happen, modules are queried, memory reads and writes occur, etc...
    // The exact meaning of this is specific to a host.  It may include which target/process is being debugged,
    // etc...
    //
    // A host may not E_NOTIMPL this method.
    //
    STDMETHOD(GetCurrentContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    // GetDefaultMetadata():
    //
    // Returns a default metadata store for the host.  The core object model relies on default metadata for
    // certain operations if specific metadata is not provided.  The display radix for a given type is one
    // example where the core model will fall back to default metadata.
    //
    // A host may E_NOTIMPL this if it does not provide a default metadata store.
    //
    STDMETHOD(GetDefaultMetadata)(
        THIS_
        _COM_Outptr_ IKeyStore** defaultMetadataStore
        ) PURE;
};

//
// IDebugHostContext:
//
// This interface represents an opaque blob of state managed by the debugger which hosts
// the data model interfaces.  This opaque blob represents the "state" of the debugger
// as it pertains to an object.  This may include, for instance, what target / process
// the object belongs to, what address space the object is in, etc...
//
// The "current" context of the debugger can be acquired via IDebugHost::GetCurrentContext
// Similarly, the context of any object can be acquired via IModelObject::GetContext.
//
// Child objects which are created by an extension should propagate the state of the parent
// whenever possible.  Only the debug host should extend or alter the context.
//
#undef INTERFACE
#define INTERFACE IDebugHostContext
DECLARE_INTERFACE_(IDebugHostContext, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostContext:

    // IsEqualTo():
    //
    // Returns whether two IDebugHostContext objects are equal by value.  Note that there
    // is no requirement for a debug host to have interface pointer equality for two contexts
    // which are equivalent.  The actual contexts can be compared through this method.
    //
    STDMETHOD(IsEqualTo)(
        THIS_
        _In_ IDebugHostContext *pContext,
        _Out_ bool *pIsEqual
        ) PURE;
};

// USE_CURRENT_HOST_CONTEXT:
//
// Methods which take an IDebugHostContext can be called with this special defined value to indicate
// to the debug host that the "current" context of the debugger should be used.  This is in lieu of
// explicitly calling IDebugHost::GetCurrentContext and explicitly passing it to the method needing
// an IDebugHostContext.
//
// Using this may be more efficient than the explicit query and pass.
//
#define USE_CURRENT_HOST_CONTEXT ((IDebugHostContext *)((ULONG_PTR)-1))

enum AddressSpaceRelation
{
    Disjoint,
    Equal,
    Overlapping,
    Subset,
    Superset
};

//
// IDebugHostContext2:
//
// Extended capabilities for working with contexts.
//
#undef INTERFACE
#define INTERFACE IDebugHostContext2
DECLARE_INTERFACE_(IDebugHostContext2, IDebugHostContext)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostContext:

    // IsEqualTo():
    //
    // Returns whether two IDebugHostContext objects are equal by value.  Note that there
    // is no requirement for a debug host to have interface pointer equality for two contexts
    // which are equivalent.  The actual contexts can be compared through this method.
    //
    STDMETHOD(IsEqualTo)(
        THIS_
        _In_ IDebugHostContext *pContext,
        _Out_ bool *pIsEqual
        ) PURE;

    //*************************************************
    // IDebugHostContext2:


    // GetAddressSpaceRelation():
    //
    // Returns the relationship between this contexts virtual memory space and another, as follows:
    //
    //   Disjoint - The two contexts share no virtual memory mappings.
    //   Equal - The two contexts share all virtual memory mappings.
    //   Overlapping - The two contexts have partially shared memory mappings.
    //   Subset - The indicated context is a strict subset of this context.
    //   Superset - The indicated context is a strict superset of this context.
    //
    STDMETHOD(GetAddressSpaceRelation)(
        THIS_
        _In_ IDebugHostContext2* pContext,
        _Out_ AddressSpaceRelation* pAddressSpaceRelation
        ) PURE;
};

// IDebugHostContextExtension:
//
// An optional "interface" on host contexts that allows for extensibility based modification.  This interface
// is *NEVER* QI'able off an IDebugHostContext.  Changes must be done via a QI for IDebugHostContextExtensibility
// and cloning an existing context (with optional additions).  Once this interface is released, it can never
// be reacquired.
//
#undef INTERFACE
#define INTERFACE IDebugHostContextExtension
DECLARE_INTERFACE_(IDebugHostContextExtension, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostContextExtension:

    // AddExtensionData:
    //
    // Adds a set of extension data to a host context.
    //
    STDMETHOD(AddExtensionData)(
        THIS_
        _In_ ULONG blobId,
        _In_ ULONG dataSize,
        _In_reads_(dataSize) PVOID data
        ) PURE;

    // FinalizeContext():
    //
    // Finalizes modifications of the host context, makes it immutable, and returns an interface to the
    // context.
    //
    STDMETHOD(FinalizeContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext **immutableContext
        ) PURE;

};

// IDebugHostContextExtensibility:
//
// An *OPTIONAL* interface for hosts to support that allows certain extensions 
#undef INTERFACE
#define INTERFACE IDebugHostContextExtensibility
DECLARE_INTERFACE_(IDebugHostContextExtensibility, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostContextExtensibility:
    //

    // HasExtensionData():
    //
    // Indicates whether a given context has a particular extension blob associated with it.
    //
    STDMETHOD_(bool, HasExtensionData)(
        THIS_
        _In_ ULONG blobId
        ) PURE;
    
    // ReadExtensionData:
    //
    // Reads a set of extension data from a host context.  This method will fail if the context does not
    // have the particular extension blob associated with it.
    //
    STDMETHOD(ReadExtensionData)(
        THIS_
        _In_ ULONG blobId,
        _In_ ULONG bufferSize,
        _Out_writes_(bufferSize) PVOID buffer
        ) PURE;

    // CloneContextForModification():
    //
    // Clones this host context and returns a one time modification interface to associate data with
    // the context.  The FinalizeContext method must be called on the resulting handle to get back to the actual 
    // cloned context.  Once that is done, the returned host context is immutable.
    //
    STDMETHOD(CloneContextForModification)(
        THIS_
        _COM_Outptr_ IDebugHostContextExtension **extensionHandle
        ) PURE;

    // CloneContextWithModification():
    //
    // Clones this host context, associates a particular extension blob with the cloned context, finalizes the
    // context, and returns an immutable interface to the newly cloned context.  To associate additional context
    // information beyond a single blob, CloneContextForModification should be used.
    //
    STDMETHOD(CloneContextWithModification)(
        THIS_
        _In_ ULONG blobId,
        _In_ ULONG dataSize,
        _In_reads_(dataSize) PVOID data,
        _COM_Outptr_ IDebugHostContext **clonedContext
        ) PURE;
};

//
// IDebugHostContextControl:
//
// This interface allows to change the "current" context (the internal state) of the debugger (IDebugHostContext)
//
// The context change can be a full change/switch (for example change the current process/thread/etx. being debugged)
// or a temporary switch. The temporary change/switch of the internal state of the debugger may alter the debugger
// in an inconsistent state and not all debugger functionality is available unless the change/switch is reverted back.

#undef INTERFACE
#define INTERFACE IDebugHostContextControl
DECLARE_INTERFACE_(IDebugHostContextControl, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostContextControl:

    // SwitchTo():
    //
    // Changes/switches the debugger engine context to IDebugHostContext (the context which IDebugHostContextControl was retrieved from).
    // This is a "full" context debuger engine switch.
    //
    STDMETHOD(SwitchTo)(
        THIS_
        ) PURE;

    // GetContextAlternator():
    //
    // Retrieves context alternator, which allows a temporary context change/switch
    //
    STDMETHOD(GetContextAlternator)(
        THIS_
        _COM_Outptr_ IDebugHostContextAlternator** contextAlternator
        ) PURE;
};

//
// IDebugHostContextAlternator:
//
// This interface allows to change the "current" context (the internal state) of the debugger (IDebugHostContext)
//
// The context change can be a full change/switch (for example change the current process/thread/etx. being debugged)
// or a temporary switch. The temporary change/switch of the internal state of the debugger may alter the debugger
// in an inconsistent state and not all debugger functionality is available unless the change/switch is reverted back.
//
#undef INTERFACE
#define INTERFACE IDebugHostContextAlternator
DECLARE_INTERFACE_(IDebugHostContextAlternator, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostContextAlternator:

    // SwitchTo():
    //
    // Changes/Switches the debugger engine context to IDebugHostContext (the context which IDebugHostContextControl was retrieved from).
    // The change/switch is a temporary and not all debugger functionality is available or may work as expected unless
    // the context is switched back
    //
    STDMETHOD(SwitchTo)(
        THIS_
        _In_ bool fullSwitch
        ) PURE;

    // SwitchBack():
    //
    // Restores the debugger engine context to its previous state before invoking SwitchTo to temporary switch to a new context.
    // the context is switched back
    //
    STDMETHOD(SwitchBack)(
        THIS_
        ) PURE;
};


//
// ErrorClass:
//
// Defines the class of error which is being reported to the host.
//
enum ErrorClass
{
    ErrorClassWarning,
    ErrorClassError
};

//
// IDebugHostErrorSink:
//
// Represents an error sink for the debug host.  Errors which occur during certain operations
// are sent to the error sink to be handled (or notify the user).
//
#undef INTERFACE
#define INTERFACE IDebugHostErrorSink
DECLARE_INTERFACE_(IDebugHostErrorSink, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostErrorSink:

    // ReportError():
    //
    // Called by the debug host to report an error.
    //
    STDMETHOD(ReportError)(
        THIS_
        _In_ ErrorClass errClass,
        _In_ HRESULT hrError,
        _In_ PCWSTR message
        ) PURE;

};

//
// IDebugHostSymbol:
//
// The base interface for any symbol (field, type, function, etc...) represented by
// the debug host.
//
#undef INTERFACE
#define INTERFACE IDebugHostSymbol
DECLARE_INTERFACE_(IDebugHostSymbol, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    // GetContext():
    //
    // Gets the debug host's context for this symbol.  Any object which is created relative to this symbol
    // can be created with this context.
    //
    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    // EnumerateChidlren():
    //
    // Enumerates all child symbols of the given type and name.  SymbolType::Symbol can be used to search
    // to search for any kind of child.
    //
    // Note that if name is nullptr, children of any name will be produced by the resulting enumerator.
    //
    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    // GetSymbolKind():
    //
    // Gets the kind of symbol that this is (e.g.: a field, a base class, a type, etc...)
    //
    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    // GetName():
    //
    // Returns the name of the symbol if the symbol has a name.  If the symbol does not have a name, an error
    // is returned.
    //
    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    // GetType():
    //
    // Returns the type (e.g.: "int *") of the symbol if the symbol has a type.  If the symbol does not have a
    // type, an error is returned.
    //
    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    // GetContainingModule():
    //
    // Returns the module which contains this symbol if the symbol has a containing module.  If the symbol does
    // not have a containing module, an error is returned.
    //
    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    // CompareAgainst():
    //
    // Compares two symbols for equality.  A host is under no obligation to ensure that there is interface pointer
    // equality for two identical symbols.  This can be used to check for equality.
    //
    // Note that presently, "comparisonFlags" is reserved.
    //
    STDMETHOD(CompareAgainst)(
        THIS_
        _In_ IDebugHostSymbol *pComparisonSymbol,
        _In_ ULONG comparisonFlags,
        _Out_ bool *pMatches
        ) PURE;

};

//
// IDebugHostSymbolEnumerator:
//
// This enumerates a set of symbols.  Such may be the set of children acquired from a call to
// the IDebugHostSymbol::EnumerateChildren method.
//
#undef INTERFACE
#define INTERFACE IDebugHostSymbolEnumerator
DECLARE_INTERFACE_(IDebugHostSymbolEnumerator, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbolEnumerator:

    // Reset():
    //
    // Resets the enumerator to its initial state.  A subsequent GetNext call will return
    // the first symbol in the set in enumerator order.
    //
    STDMETHOD(Reset)(
        THIS
        ) PURE;

    // GetNext():
    //
    // Moves the iterator forward and fetches the next symbol in the set.
    //
    // E_BOUNDS will be returned when the enumerator hits the end of the set.
    //
    STDMETHOD(GetNext)(
        THIS_
        _COM_Outptr_ IDebugHostSymbol** symbol
        ) PURE;
};

#undef INTERFACE
#define INTERFACE IDebugHostSymbolSubstitutionEnumerator
DECLARE_INTERFACE_(IDebugHostSymbolSubstitutionEnumerator, IDebugHostSymbolEnumerator)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbolEnumerator:
    //

    // Reset():
    //
    // Resets the enumerator to its initial state.  A subsequent GetNext call will return
    // the first symbol in the set in enumerator order.
    //
    STDMETHOD(Reset)(
        THIS
        ) PURE;

    // GetNext():
    //
    // Moves the iterator forward and fetches the next symbol in the set.
    //
    // E_BOUNDS will be returned when the enumerator hits the end of the set.
    //
    STDMETHOD(GetNext)(
        THIS_
        _COM_Outptr_ IDebugHostSymbol** symbol
        ) PURE;

    //*************************************************
    // IDebugHostSymbolSubstitutionEnumerator:
    //

    // GetNextWithSubstitutionText():
    //
    // Moves the iterator forward and fetches both the next symbol in the set and the textual representation
    // of that symbol as appropriate in its given context.
    //
    STDMETHOD(GetNextWithSubstitutionText)(
        THIS_
        _COM_Outptr_opt_result_maybenull_ IDebugHostSymbol** symbol,
        _Out_opt_ BSTR *symbolText
        ) PURE;
};

//
// IDebugHostModule:
//
// A specialization of IDebugHostSymbol representing a module (e.g.: a DLL or executable).
//
#undef INTERFACE
#define INTERFACE IDebugHostModule
DECLARE_INTERFACE_(IDebugHostModule, IDebugHostSymbol)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    //*************************************************
    // IDebugHostModule:

    // GetImageName():
    //
    // Returns the image name of the module.  If "allowPath" is set, the resulting imageName **MAY OR MAY NOT**
    // include the path to the image (depending on the particular debug host).  If the "allowPath" is not set,
    // the resulting imageName will only include the name.
    //
    STDMETHOD(GetImageName)(
        THIS_
        _In_ bool allowPath,
        _Out_ BSTR* imageName
        ) PURE;

    // GetBaseLocation():
    //
    // Returns the location of the base of the module in the address space of the debug target.
    //
    STDMETHOD(GetBaseLocation)(
        THIS_
        _Out_ Location* moduleBaseLocation
        ) PURE;

    // GetVersion():
    //
    // Returns the file and product version of the module (assuming they can be read).  If a given version
    // is requested (via a non-nullptr output pointer) and it cannot be read, an appropriate error will be
    // returned.
    //
    STDMETHOD(GetVersion)(
        THIS_
        _Out_opt_ ULONG64* fileVersion,
        _Out_opt_ ULONG64* productVersion
        ) PURE;

    // FindTypeByName():
    //
    // Finds a type (e.g.: "int *") which is defined within the module by name.  This method may return a valid
    // IDebugHostType which would never be returned via an explicit recursion of children of the module.  The
    // debug host may allow the creation of "derivative types" of things not in the module in question
    // (e.g.: a "MyStruct **" where "MyStruct" is never used in pointer form).
    //
    STDMETHOD(FindTypeByName)(
        THIS_
        _In_z_ PCWSTR typeName,
        _Out_ IDebugHostType** type
        ) PURE;

    // FindSymbolByRVA():
    //
    // Finds a single symbol at the given relative virtual address within the module.  If there is not a single
    // symbol at the supplied RVA, an error will be returned.
    //
    STDMETHOD(FindSymbolByRVA)(
        THIS_
        _In_ ULONG64 rva,
        _Out_ IDebugHostSymbol** symbol
        ) PURE;

    // FindSymbolByName():
    //
    // Finds a single symbol by name within the module.  If there is not a single symbol matching the given name,
    // an error will be returned.
    //
    STDMETHOD(FindSymbolByName)(
        THIS_
        _In_z_ PCWSTR symbolName,
        _Out_ IDebugHostSymbol** symbol
        ) PURE;
};

//
// IDebugHostModule2:
//
// A specialization of IDebugHostSymbol representing a module (e.g.: a DLL or executable).
//
#undef INTERFACE
#define INTERFACE IDebugHostModule2
DECLARE_INTERFACE_(IDebugHostModule2, IDebugHostModule)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    //*************************************************
    // IDebugHostModule:

    STDMETHOD(GetImageName)(
        THIS_
        _In_ bool allowPath,
        _Out_ BSTR* imageName
        ) PURE;

    STDMETHOD(GetBaseLocation)(
        THIS_
        _Out_ Location* moduleBaseLocation
        ) PURE;

    STDMETHOD(GetVersion)(
        THIS_
        _Out_opt_ ULONG64* fileVersion,
        _Out_opt_ ULONG64* productVersion
        ) PURE;

    STDMETHOD(FindTypeByName)(
        THIS_
        _In_z_ PCWSTR typeName,
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(FindSymbolByRVA)(
        THIS_
        _In_ ULONG64 rva,
        _Out_ IDebugHostSymbol** symbol
        ) PURE;

    STDMETHOD(FindSymbolByName)(
        THIS_
        _In_z_ PCWSTR symbolName,
        _Out_ IDebugHostSymbol** symbol
        ) PURE;

    //*************************************************
    // IDebugHostModule2:

    // FindContainingSymbolByRVA():
    //
    // Finds a single symbol whose size indicates that the given relative virtual address is contained within it.  If there is not a single
    // symbol at the supplied RVA, an error will be returned.
    //
    // The offset to the symbol will be returned as well.
    //
    STDMETHOD(FindContainingSymbolByRVA)(
        THIS_
        _In_ ULONG64 rva,
        _Out_ IDebugHostSymbol** symbol,
        _Out_ ULONG64 *offset
        ) PURE;
};

//
// IDebugHostModule3:
//
// A specialization of IDebugHostSymbol representing a module (e.g.: a DLL or executable).
//
#undef INTERFACE
#define INTERFACE IDebugHostModule3
DECLARE_INTERFACE_(IDebugHostModule3, IDebugHostModule2)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    //*************************************************
    // IDebugHostModule:

    STDMETHOD(GetImageName)(
        THIS_
        _In_ bool allowPath,
        _Out_ BSTR* imageName
        ) PURE;

    STDMETHOD(GetBaseLocation)(
        THIS_
        _Out_ Location* moduleBaseLocation
        ) PURE;

    STDMETHOD(GetVersion)(
        THIS_
        _Out_opt_ ULONG64* fileVersion,
        _Out_opt_ ULONG64* productVersion
        ) PURE;

    STDMETHOD(FindTypeByName)(
        THIS_
        _In_z_ PCWSTR typeName,
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(FindSymbolByRVA)(
        THIS_
        _In_ ULONG64 rva,
        _Out_ IDebugHostSymbol** symbol
        ) PURE;

    STDMETHOD(FindSymbolByName)(
        THIS_
        _In_z_ PCWSTR symbolName,
        _Out_ IDebugHostSymbol** symbol
        ) PURE;

    //*************************************************
    // IDebugHostModule2:

    // FindContainingSymbolByRVA():
    //
    // Finds a single symbol whose size indicates that the given relative virtual address is contained within it.  If there is not a single
    // symbol at the supplied RVA, an error will be returned.
    //
    // The offset to the symbol will be returned as well.
    //
    STDMETHOD(FindContainingSymbolByRVA)(
        THIS_
        _In_ ULONG64 rva,
        _Out_ IDebugHostSymbol** symbol,
        _Out_ ULONG64 *offset
        ) PURE;

    //*************************************************
    // IDebugHostModule3:

    // GetRange():
    //
    // Returns the beginning and ending VA of the module in memory.
    //
    STDMETHOD(GetRange)(
        THIS_
        _Out_ Location* moduleStart,
        _Out_ Location* moduleEnd
        ) PURE;

};

#undef INTERFACE
#define INTERFACE IDebugHostModule4
DECLARE_INTERFACE_(IDebugHostModule4, IDebugHostModule3)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    //*************************************************
    // IDebugHostModule:

    STDMETHOD(GetImageName)(
        THIS_
        _In_ bool allowPath,
        _Out_ BSTR* imageName
        ) PURE;

    STDMETHOD(GetBaseLocation)(
        THIS_
        _Out_ Location* moduleBaseLocation
        ) PURE;

    STDMETHOD(GetVersion)(
        THIS_
        _Out_opt_ ULONG64* fileVersion,
        _Out_opt_ ULONG64* productVersion
        ) PURE;

    STDMETHOD(FindTypeByName)(
        THIS_
        _In_z_ PCWSTR typeName,
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(FindSymbolByRVA)(
        THIS_
        _In_ ULONG64 rva,
        _Out_ IDebugHostSymbol** symbol
        ) PURE;

    STDMETHOD(FindSymbolByName)(
        THIS_
        _In_z_ PCWSTR symbolName,
        _Out_ IDebugHostSymbol** symbol
        ) PURE;

    //*************************************************
    // IDebugHostModule2:

    // FindContainingSymbolByRVA():
    //
    // Finds a single symbol whose size indicates that the given relative virtual address is contained within it.  If there is not a single
    // symbol at the supplied RVA, an error will be returned.
    //
    // The offset to the symbol will be returned as well.
    //
    STDMETHOD(FindContainingSymbolByRVA)(
        THIS_
        _In_ ULONG64 rva,
        _Out_ IDebugHostSymbol** symbol,
        _Out_ ULONG64 *offset
        ) PURE;

    //*************************************************
    // IDebugHostModule3:

    STDMETHOD(GetRange)(
        THIS_
        _Out_ Location* moduleStart,
        _Out_ Location* moduleEnd
        ) PURE;

    //*************************************************
    // IDebugHostModule4:
    //

    // FindTypeByName2():
    //
    // Finds a type by name within the module.  If an enclosing symbol is given, the type is looked for by name
    // within that enclosing symbol; otherwise, the type is looked for globally.
    //
    STDMETHOD(FindTypeByName2)(
        THIS_
        _In_opt_ IDebugHostSymbol *pEnclosingSymbol,
        _In_z_ PCWSTR typeName,
        _Out_ IDebugHostType** type
        ) PURE;
};

// KnownCompiler:
//
// Identifies a set of well known compilers that we know something about.
//
enum KnownCompiler
{
    CompilerUnknown,
    CompilerMSVC,
    CompilerGCC,
    CompilerClang,
    CompilerRustC
};

#undef INTERFACE
#define INTERFACE IDebugHostModule5
DECLARE_INTERFACE_(IDebugHostModule5, IDebugHostModule4)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    //*************************************************
    // IDebugHostModule:

    STDMETHOD(GetImageName)(
        THIS_
        _In_ bool allowPath,
        _Out_ BSTR* imageName
        ) PURE;

    STDMETHOD(GetBaseLocation)(
        THIS_
        _Out_ Location* moduleBaseLocation
        ) PURE;

    STDMETHOD(GetVersion)(
        THIS_
        _Out_opt_ ULONG64* fileVersion,
        _Out_opt_ ULONG64* productVersion
        ) PURE;

    STDMETHOD(FindTypeByName)(
        THIS_
        _In_z_ PCWSTR typeName,
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(FindSymbolByRVA)(
        THIS_
        _In_ ULONG64 rva,
        _Out_ IDebugHostSymbol** symbol
        ) PURE;

    STDMETHOD(FindSymbolByName)(
        THIS_
        _In_z_ PCWSTR symbolName,
        _Out_ IDebugHostSymbol** symbol
        ) PURE;

    //*************************************************
    // IDebugHostModule2:

    // FindContainingSymbolByRVA():
    //
    // Finds a single symbol whose size indicates that the given relative virtual address is contained within it.  If there is not a single
    // symbol at the supplied RVA, an error will be returned.
    //
    // The offset to the symbol will be returned as well.
    //
    STDMETHOD(FindContainingSymbolByRVA)(
        THIS_
        _In_ ULONG64 rva,
        _Out_ IDebugHostSymbol** symbol,
        _Out_ ULONG64 *offset
        ) PURE;

    //*************************************************
    // IDebugHostModule3:

    STDMETHOD(GetRange)(
        THIS_
        _Out_ Location* moduleStart,
        _Out_ Location* moduleEnd
        ) PURE;

    //*************************************************
    // IDebugHostModule4:
    //

    // FindTypeByName2():
    //
    // Finds a type by name within the module.  If an enclosing symbol is given, the type is looked for by name
    // within that enclosing symbol; otherwise, the type is looked for globally.
    //
    STDMETHOD(FindTypeByName2)(
        THIS_
        _In_opt_ IDebugHostSymbol *pEnclosingSymbol,
        _In_z_ PCWSTR typeName,
        _Out_ IDebugHostType** type
        ) PURE;

    //*************************************************
    // IDebugHostModule5:
    //

    // GetPrimaryCompilerInformation():
    //
    // Returns information about what might be considered the "primary compiler" which produced the module.
    // Such information may, for instance, be used to understand something about how symbols are formatted
    // by that compiler, etc...  While this may differ for each compilation unit / compiland (e.g.: there may
    // be assembly code linked in, etc...), this should return the "primary" or most significant one for
    // non-assembly CUs.
    //
    // Note that the "compiler string" returned may be a compiler name or may include additional information
    // (e.g.: command line arguments, etc...).  That depends on the underlying implementation.
    // 
    // It is legal for a debug host to E_NOTIMPL this.
    //
    STDMETHOD(GetPrimaryCompilerInformation)(
        THIS_
        _Out_ KnownCompiler *pCompilerId,
        _Out_opt_ BSTR *pPrimaryCompilerString
        ) PURE;
};


//
// ArrayDimension:
//
// Defines the memory layout of one dimension of an array.
//
struct ArrayDimension
{
    // The lower bounds of the array.  For C style zero based arrays, this will always be zero.  There is no
    // uniform restriction that all arrays represented by these interfaces are zero based.
    LONG64 LowerBound;

    // Defines the length of the dimension.  The dimension is considered to be of the form [LowerBound, LowerBound + Length)
    ULONG64 Length;

    // Defines how many bytes to move forward in memory to walk from index N of the dimension to index N + 1
    ULONG64 Stride;
};

//
// IDebugHostType:
//
// A specialization of IDebugHostSymbol representing a type (e.g.: "MyStruct *")
//
#undef INTERFACE
#define INTERFACE IDebugHostType
DECLARE_INTERFACE_(IDebugHostType, IDebugHostSymbol)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    //*************************************************
    // IDebugHostType:

    // GetKind():
    //
    // Returns the kind of type (e.g.: a pointer, an array, an intrinsic, etc...)
    //
    STDMETHOD(GetTypeKind)(
        THIS_
        _Out_ TypeKind *kind
        ) PURE;

    // GetSize():
    //
    // Returns the overall size of the type as laid out in memory.
    //
    STDMETHOD(GetSize)(
        THIS_
        _Out_ ULONG64* size
        ) PURE;

    // GetBaseType():
    //
    // If the type is a derivative of another single type (e.g.: as "MyStruct *" is derived from "MyStruct"),
    // this returns the base type of the derivation.  For pointers, this would return the type pointed to.
    // For arrays, this would return what the array is an array of.  If the type is not such a derivative
    // type, an error is returned.
    //
    // Not that this method has nothing to do with C++ base classes.  Such are symbols (IDebugHostBaseClass)
    // which can be enumerated from the derived class via a call to EnumerateChildren.
    //
    STDMETHOD(GetBaseType)(
        THIS_
        _Out_ IDebugHostType** baseType
        ) PURE;

    // GetHashCode():
    //
    // Returns a 32-bit hash code for the type.  With the exception of a global match (e.g.: a type signature
    // like "*" if permitted by the host), any type instance which can match a particular type signature must
    // return the same hash code.
    //
    // This is used in conjunction with type signatures in order to match type signatures to type instances.
    //
    STDMETHOD(GetHashCode)(
        THIS_
        _Out_ ULONG* hashCode
        ) PURE;

    // GetIntrinsicType():
    //
    // If the type kind as reported by GetTypeKind is an intrinsic, this returns more information about the
    // particular kind of intrinsic.
    //
    // The returned "intrinsicKind" indicates things like whether it is a bool, integer, floating point, etc...
    // but not necessarily the size.  The returned "carrierType" indicates how this intrinsic value is
    // packed into a VARIANT structure.  The combination of this information indicates the full set
    // of information about the intrinsic.
    //
    STDMETHOD(GetIntrinsicType)(
        THIS_
        _Out_opt_ IntrinsicKind *intrinsicKind,
        _Out_opt_ VARTYPE *carrierType
        ) PURE;

    //*************************************************
    // Bitfield Information:
    //
    // The following methods only apply to types which are bitfields:
    //

    // GetBitField():
    //
    // If the type is a bit field, this returns the numeric position of the least significant bit of the
    // field and the length of the field.  Bit positions (lsbOfField + lengthOfField : lsbOfField] define
    // the bit position.
    //
    STDMETHOD(GetBitField)(
        THIS_
        _Out_ ULONG* lsbOfField,
        _Out_ ULONG* lengthOfField
        ) PURE;


    //*************************************************
    // Pointer Information (GetKind returns TypePointer):
    //
    // The following methods only apply to types which are pointers (or create
    // derivative types which are pointers)
    //

    // GetPointerKind():
    //
    // Returns what kind of pointer the type is (e.g.: a standard pointer, a pointer to member,
    // a reference, an r-value reference, a C++/CX hat, etc...)
    //
    STDMETHOD(GetPointerKind)(
        THIS_
        _Out_ PointerKind* pointerKind
        ) PURE;

    // GetMemberType():
    //
    // If the pointer is a pointer-to-class-member, this returns the type of such class.
    //
    STDMETHOD(GetMemberType)(
        THIS_
        _Out_ IDebugHostType** memberType
        ) PURE;

    // CreatePointerTo():
    //
    // For any given type, this returns a new IDebugHostType which is a pointer to this type.
    // The kind of pointer is supplied by the "kind" argument.
    //
    STDMETHOD(CreatePointerTo)(
        THIS_
        _In_ PointerKind kind,
        _COM_Outptr_ IDebugHostType** newType
        ) PURE;

    //*************************************************
    // Array Information (GetKind returns TypeArray):
    //
    // The following methods only apply to types which are arrays (or create
    // derivative types which are arrays)
    //

    // GetArrayDimensionality():
    //
    // Returns the dimensionality of the array.  There is no guarantee that every array type representable by
    // these interfaces is a standard zero-based one dimensional array as is standard in C.
    //
    STDMETHOD(GetArrayDimensionality)(
        THIS_
        _Out_ ULONG64* arrayDimensionality
        ) PURE;

    // GetArrayDimensions():
    //
    // Fills in information about each dimension of the array including its lower bound, length, and stride.
    // This method should not be called on TypeExtendedArray.  Methods in IDebugHostType4 should be utilized.
    //
    STDMETHOD(GetArrayDimensions)(
        THIS_
        _In_ ULONG64 dimensions,
        _Out_writes_(dimensions) ArrayDimension *pDimensions
        ) PURE;

    // CreateArrayOf():
    //
    // For any given type, this returns a new IDebugHostType which is an array of this type.
    // The dimensions of the array must be supplied via the "dimensions" and "pDimensions" arguments.
    // This method should not be called on TypeExtendedArray.  Methods in IDebugHostType4 should be utilized.
    //
    STDMETHOD(CreateArrayOf)(
        THIS_
        _In_ ULONG64 dimensions,
        _In_reads_(dimensions) ArrayDimension *pDimensions,
        _COM_Outptr_ IDebugHostType** newType
        ) PURE;

    //*************************************************
    // Function Information (GetKind returns TypeFunction):
    //
    // The following methods only apply to types which are functions (or create
    // derivative types which are functions)
    //

    // GetFunctionCallingConvention():
    //
    // Returns the calling convention of the function (e.g.: STDCALL, FASTCALL, etc...)
    //
    STDMETHOD(GetFunctionCallingConvention)(
        THIS_
        _Out_ CallingConventionKind* conventionKind
        ) PURE;

    // GetFunctionReturnType():
    //
    // Gets the return type of the function as an IDebugHostType.
    //
    STDMETHOD(GetFunctionReturnType)(
        THIS_
        _COM_Outptr_ IDebugHostType** returnType
        ) PURE;

    // GetFunctionParameterTypeCount():
    //
    // Gets the number of parameter types that the function takes.
    //
    STDMETHOD(GetFunctionParameterTypeCount)(
        THIS_
        _Out_ ULONG64* count
        ) PURE;

    // GetFunctionParameterTypeAt():
    //
    // Returns the type of the "i"-th argument to the function as a new IDebugHostType.
    //
    STDMETHOD(GetFunctionParameterTypeAt)(
        THIS_
        _In_ ULONG64 i,
        _Out_ IDebugHostType** parameterType
        ) PURE;

    //*************************************************
    // Template Information:
    //
    // The following methods only apply to types which are templates or generics (or create
    // derivative types which are templates or generics)
    //

    // IsGeneric():
    //
    // Returns whether the type is a generic or template.
    //
    STDMETHOD(IsGeneric)(
        THIS_
        _Out_ bool* isGeneric
        ) PURE;

    // GetGenericArgumentCount():
    //
    // Returns the number of arguments to the generic/template.  The returned value must be greater
    // than zero.
    //
    STDMETHOD(GetGenericArgumentCount)(
        THIS_
        _Out_ ULONG64* argCount
        ) PURE;

    // GetGenericArgumentAt():
    //
    // For the "i"-th generic argument to the generic/template, this returns a new IDebugHostSymbol
    // which represents that argument.  For templates, this is most often an IDebugHostType; however --
    // it may be an IDebugHostConstant for non-template type arguments.
    //
    // Note that it is possible for some compiler generated generics and templates that this method
    // will fail.
    //
    STDMETHOD(GetGenericArgumentAt)(
        THIS_
        _In_ ULONG64 i,
        _Out_ IDebugHostSymbol** argument
        ) PURE;

};

// VarArgsKind:
//
// Defines the style of variable arguments that a function definition takes.
//
enum VarArgsKind
{
    // The function does not take any variable arguments
    VarArgsNone,

    // The function is a C-style varargs function ( returntype(arg1, arg2, ...) ).  The number of arguments
    // reported by the function does not include the ellipsis ('...') argument.  Any variable argument
    // passing occurs after the number of arguments returned by GetFunctionParameterTypeCount(...).
    //
    VarArgsCStyle
};

//
// IDebugHostType2:
//
// Additional type information capabilities provided in addition to those of IDebugHostType.
//
#undef INTERFACE
#define INTERFACE IDebugHostType2
DECLARE_INTERFACE_(IDebugHostType2, IDebugHostType)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    //*************************************************
    // IDebugHostType:

    STDMETHOD(GetTypeKind)(
        THIS_
        _Out_ TypeKind *kind
        ) PURE;

    STDMETHOD(GetSize)(
        THIS_
        _Out_ ULONG64* size
        ) PURE;

    STDMETHOD(GetBaseType)(
        THIS_
        _Out_ IDebugHostType** baseType
        ) PURE;

    STDMETHOD(GetHashCode)(
        THIS_
        _Out_ ULONG* hashCode
        ) PURE;

    STDMETHOD(GetIntrinsicType)(
        THIS_
        _Out_opt_ IntrinsicKind *intrinsicKind,
        _Out_opt_ VARTYPE *carrierType
        ) PURE;

    //*************************************************
    // Bitfield Information:
    //
    // The following methods only apply to types which are bitfields:
    //

    STDMETHOD(GetBitField)(
        THIS_
        _Out_ ULONG* lsbOfField,
        _Out_ ULONG* lengthOfField
        ) PURE;

    //*************************************************
    // Pointer Information (GetKind returns TypePointer):
    //
    // The following methods only apply to types which are pointers (or create
    // derivative types which are pointers)
    //

    STDMETHOD(GetPointerKind)(
        THIS_
        _Out_ PointerKind* pointerKind
        ) PURE;

    STDMETHOD(GetMemberType)(
        THIS_
        _Out_ IDebugHostType** memberType
        ) PURE;

    STDMETHOD(CreatePointerTo)(
        THIS_
        _In_ PointerKind kind,
        _COM_Outptr_ IDebugHostType** newType
        ) PURE;

    //*************************************************
    // Array Information (GetKind returns TypeArray):
    //
    // The following methods only apply to types which are arrays (or create
    // derivative types which are arrays)
    //

    STDMETHOD(GetArrayDimensionality)(
        THIS_
        _Out_ ULONG64* arrayDimensionality
        ) PURE;

    STDMETHOD(GetArrayDimensions)(
        THIS_
        _In_ ULONG64 dimensions,
        _Out_writes_(dimensions) ArrayDimension *pDimensions
        ) PURE;

    STDMETHOD(CreateArrayOf)(
        THIS_
        _In_ ULONG64 dimensions,
        _In_reads_(dimensions) ArrayDimension *pDimensions,
        _COM_Outptr_ IDebugHostType** newType
        ) PURE;

    //*************************************************
    // Function Information (GetKind returns TypeFunction):
    //
    // The following methods only apply to types which are functions (or create
    // derivative types which are functions)
    //

    STDMETHOD(GetFunctionCallingConvention)(
        THIS_
        _Out_ CallingConventionKind* conventionKind
        ) PURE;

    STDMETHOD(GetFunctionReturnType)(
        THIS_
        _COM_Outptr_ IDebugHostType** returnType
        ) PURE;

    STDMETHOD(GetFunctionParameterTypeCount)(
        THIS_
        _Out_ ULONG64* count
        ) PURE;

    STDMETHOD(GetFunctionParameterTypeAt)(
        THIS_
        _In_ ULONG64 i,
        _Out_ IDebugHostType** parameterType
        ) PURE;

    //*************************************************
    // Template Information:
    //
    // The following methods only apply to types which are templates or generics (or create
    // derivative types which are templates or generics)
    //

    STDMETHOD(IsGeneric)(
        THIS_
        _Out_ bool* isGeneric
        ) PURE;

    STDMETHOD(GetGenericArgumentCount)(
        THIS_
        _Out_ ULONG64* argCount
        ) PURE;

    STDMETHOD(GetGenericArgumentAt)(
        THIS_
        _In_ ULONG64 i,
        _Out_ IDebugHostSymbol** argument
        ) PURE;

    //**************************************************************************
    // IDebugHostType2:
    //
    // The following methods are only available in the v2 IDebugHostType interface.

    //*************************************************
    // Typedef Information:
    //
    // Typedef types are, for the vast majority of calls, indistinguishable from the types they are definitions
    // against.  In the event that the true typedef information needs to be known, the following APIs
    // apply.
    //

    // IsTypedef():
    //
    // Returns whether the type is a typedef for another type or not.
    //
    STDMETHOD(IsTypedef)(
        THIS_
        _Out_ bool* isTypedef
        ) PURE;

    // GetTypedefBaseType():
    //
    // If IsTypedef returns true, this will return the type that a typedef refers to.  Note that the base
    // type may be another typedef.  It is entirely possible to have a chain of definitions.  If the caller
    // wishes to acquire the first non-typedef type of a chain, GetTypedefFinalBaseType is the appropriate call.
    //
    STDMETHOD(GetTypedefBaseType)(
        THIS_
        _Out_ IDebugHostType2** baseType
        ) PURE;

    // GetTypedefFinalBaseType():
    //
    // If IsTypedef returns true, this will return the bottom-most type of a typedef chain.  That is, the first
    // type in the chain which is NOT a typedef.  This is the type which is implicitly used for most
    // non-typedef IDebugHostType::* methods on a typedef type.
    //
    STDMETHOD(GetTypedefFinalBaseType)(
        THIS_
        _Out_ IDebugHostType2** finalBaseType
        ) PURE;

    //*************************************************
    // Extended Function Information:
    //
    // The following methods indicate additional information about function types that is only available through
    // IDebugHostType2.
    //

    // GetFunctionVarArgsKind():
    //
    // Indicates whether and what kind of variable arguments a function takes.
    //
    STDMETHOD(GetFunctionVarArgsKind)(
        THIS_
        _Out_ VarArgsKind* varArgsKind
        ) PURE;

    // GetFunctionInstancePointerType():
    //
    // Indicates what the type of the instance ("this") pointer passed to the function is.  This method will fail
    // if the function is not an instance method on a class.
    //
    STDMETHOD(GetFunctionInstancePointerType)(
        THIS_
        _Out_ IDebugHostType2** instancePointerType
        ) PURE;

};

//
// IDebugHostType3:
//
// Additional type information capabilities provided in addition to those of IDebugHostType.
//
#undef INTERFACE
#define INTERFACE IDebugHostType3
DECLARE_INTERFACE_(IDebugHostType3, IDebugHostType2)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    //*************************************************
    // IDebugHostType:

    STDMETHOD(GetTypeKind)(
        THIS_
        _Out_ TypeKind *kind
        ) PURE;

    STDMETHOD(GetSize)(
        THIS_
        _Out_ ULONG64* size
        ) PURE;

    STDMETHOD(GetBaseType)(
        THIS_
        _Out_ IDebugHostType** baseType
        ) PURE;

    STDMETHOD(GetHashCode)(
        THIS_
        _Out_ ULONG* hashCode
        ) PURE;

    STDMETHOD(GetIntrinsicType)(
        THIS_
        _Out_opt_ IntrinsicKind *intrinsicKind,
        _Out_opt_ VARTYPE *carrierType
        ) PURE;

    //*************************************************
    // Bitfield Information:
    //
    // The following methods only apply to types which are bitfields:
    //

    STDMETHOD(GetBitField)(
        THIS_
        _Out_ ULONG* lsbOfField,
        _Out_ ULONG* lengthOfField
        ) PURE;

    //*************************************************
    // Pointer Information (GetKind returns TypePointer):
    //
    // The following methods only apply to types which are pointers (or create
    // derivative types which are pointers)
    //

    STDMETHOD(GetPointerKind)(
        THIS_
        _Out_ PointerKind* pointerKind
        ) PURE;

    STDMETHOD(GetMemberType)(
        THIS_
        _Out_ IDebugHostType** memberType
        ) PURE;

    STDMETHOD(CreatePointerTo)(
        THIS_
        _In_ PointerKind kind,
        _COM_Outptr_ IDebugHostType** newType
        ) PURE;

    //*************************************************
    // Array Information (GetKind returns TypeArray):
    //
    // The following methods only apply to types which are arrays (or create
    // derivative types which are arrays)
    //

    STDMETHOD(GetArrayDimensionality)(
        THIS_
        _Out_ ULONG64* arrayDimensionality
        ) PURE;

    STDMETHOD(GetArrayDimensions)(
        THIS_
        _In_ ULONG64 dimensions,
        _Out_writes_(dimensions) ArrayDimension *pDimensions
        ) PURE;

    STDMETHOD(CreateArrayOf)(
        THIS_
        _In_ ULONG64 dimensions,
        _In_reads_(dimensions) ArrayDimension *pDimensions,
        _COM_Outptr_ IDebugHostType** newType
        ) PURE;

    //*************************************************
    // Function Information (GetKind returns TypeFunction):
    //
    // The following methods only apply to types which are functions (or create
    // derivative types which are functions)
    //

    STDMETHOD(GetFunctionCallingConvention)(
        THIS_
        _Out_ CallingConventionKind* conventionKind
        ) PURE;

    STDMETHOD(GetFunctionReturnType)(
        THIS_
        _COM_Outptr_ IDebugHostType** returnType
        ) PURE;

    STDMETHOD(GetFunctionParameterTypeCount)(
        THIS_
        _Out_ ULONG64* count
        ) PURE;

    STDMETHOD(GetFunctionParameterTypeAt)(
        THIS_
        _In_ ULONG64 i,
        _Out_ IDebugHostType** parameterType
        ) PURE;

    //*************************************************
    // Template Information:
    //
    // The following methods only apply to types which are templates or generics (or create
    // derivative types which are templates or generics)
    //

    STDMETHOD(IsGeneric)(
        THIS_
        _Out_ bool* isGeneric
        ) PURE;

    STDMETHOD(GetGenericArgumentCount)(
        THIS_
        _Out_ ULONG64* argCount
        ) PURE;

    STDMETHOD(GetGenericArgumentAt)(
        THIS_
        _In_ ULONG64 i,
        _Out_ IDebugHostSymbol** argument
        ) PURE;

    //**************************************************************************
    // IDebugHostType2:
    //
    // The following methods are only available in the v2 IDebugHostType interface.

    //*************************************************
    // Typedef Information:
    //
    // Typedef types are, for the vast majority of calls, indistinguishable from the types they are definitions
    // against.  In the event that the true typedef information needs to be known, the following APIs
    // apply.
    //

    // IsTypedef():
    //
    // Returns whether the type is a typedef for another type or not.
    //
    STDMETHOD(IsTypedef)(
        THIS_
        _Out_ bool* isTypedef
        ) PURE;

    // GetTypedefBaseType():
    //
    // If IsTypedef returns true, this will return the type that a typedef refers to.  Note that the base
    // type may be another typedef.  It is entirely possible to have a chain of definitions.  If the caller
    // wishes to acquire the first non-typedef type of a chain, GetTypedefFinalBaseType is the appropriate call.
    //
    STDMETHOD(GetTypedefBaseType)(
        THIS_
        _Out_ IDebugHostType2** baseType
        ) PURE;

    // GetTypedefFinalBaseType():
    //
    // If IsTypedef returns true, this will return the bottom-most type of a typedef chain.  That is, the first
    // type in the chain which is NOT a typedef.  This is the type which is implicitly used for most
    // non-typedef IDebugHostType::* methods on a typedef type.
    //
    STDMETHOD(GetTypedefFinalBaseType)(
        THIS_
        _Out_ IDebugHostType2** finalBaseType
        ) PURE;

    //*************************************************
    // Extended Function Information:
    //
    // The following methods indicate additional information about function types that is only available through
    // IDebugHostType2.
    //

    // GetFunctionVarArgsKind():
    //
    // Indicates whether and what kind of variable arguments a function takes.
    //
    STDMETHOD(GetFunctionVarArgsKind)(
        THIS_
        _Out_ VarArgsKind* varArgsKind
        ) PURE;

    // GetFunctionInstancePointerType():
    //
    // Indicates what the type of the instance ("this") pointer passed to the function is.  This method will fail
    // if the function is not an instance method on a class.
    //
    STDMETHOD(GetFunctionInstancePointerType)(
        THIS_
        _Out_ IDebugHostType2** instancePointerType
        ) PURE;

    //**************************************************************************
    // IDebugHostType3:
    //
    // The following methods are only available in the IDebugHostType3 interface.

    // GetContainingType():
    //
    // Returns the type of the containing parent (containing this symbol)
    //
    STDMETHOD(GetContainingType)(
        THIS_
        _Out_ IDebugHostType3** containingParentType
        ) PURE;


};

// ExtendedArrayDimension:
//
// Defines the memory layout of one dimension of an extended array.  This must be sufficient to describe
// the array layout of a CLI (ECMA-335) array.
//
enum ExtendedArrayDimensionFlags
{
    //
    // Indicates that the "Length" field of the array dimension is an offset from the base address of the array
    // as to where to find a dynamic size
    //
    ExtendedArrayLengthIsOffset32 = 0x00000001,
    ExtendedArrayLengthIsOffset64 = 0x00000002,
    ExtendedArrayLengthIsOffset   = 0x00000003,

    //
    // Indicates that the "LowerBound" field of the array dimension is an offset from the base address of the
    // array as to where to find a dynamic bound.
    //
    ExtendedArrayLowerBoundIsOffset32 = 0x00000004,
    ExtendedArrayLowerBoundIsOffset64 = 0x00000008,
    ExtendedArrayLowerBoundIsOffset   = 0x0000000C,

    //
    // Indicates that the "Stride" field of the array dimension is an offset from the base address of the
    // array as to where to find a dynamic stride
    //
    ExtendedArrayStrideIsOffset32 = 0x00000010,
    ExtendedArrayStrideIsOffset64 = 0x00000020,
    ExtendedArrayStrideIsOffset   = 0x00000030,

    //
    // Indicates that the "Stride" field is computed from the element size and the computed sizes of each
    // dimension as indicated by other fields.
    //
    // Next indicates that the stride of this dimension is based on the stride of the next (e.g.: dim[0] is the largest)
    // Previous indicates that the stride of this dimension is based on the stride of the previous (e.g.: dim[0] is the smallest)
    //
    ExtendedArrayStrideIsComputedByNextRank     = 0x00000040,
    ExtendedArrayStrideIsComputedByPreviousRank = 0x00000080,
    ExtendedArrayStrideIsComputed               = 0x000000C0

};

struct ExtendedArrayDimension
{
    // Information about how to interpret the remainder of the information in the array dimension
    ULONG64 DimensionFlags;

    // The lower bounds of the array.  For C style zero based arrays, this will always be zero.  There is no
    // uniform restriction that all arrays represented by these interfaces are zero based.
    LONG64 LowerBound;

    // Defines the length of the dimension.  The dimension is considered to be of the form [LowerBound, LowerBound + Length)
    ULONG64 Length;

    // Defines how many bytes to move forward in memory to walk from index N of the dimension to index N + 1
    ULONG64 Stride;
};

// UDTKind:
//
// Defines the nature of the UDT in question.
//
enum UDTKind
{
    // UDT is a structure
    UDTStruct,

    // UDT is a class
    UDTClass,

    // UDT is a union
    UDTUnion,

    // UDT is an interface
    UDTInterface,

    // UDT is a tagged union
    UDTTaggedUnion
};

//
// IDebugHostType4:
//
// Additional type information capabilities provided in addition to those of IDebugHostType.
//
#undef INTERFACE
#define INTERFACE IDebugHostType4
DECLARE_INTERFACE_(IDebugHostType4, IDebugHostType3)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    //*************************************************
    // IDebugHostType:

    STDMETHOD(GetTypeKind)(
        THIS_
        _Out_ TypeKind *kind
        ) PURE;

    STDMETHOD(GetSize)(
        THIS_
        _Out_ ULONG64* size
        ) PURE;

    STDMETHOD(GetBaseType)(
        THIS_
        _Out_ IDebugHostType** baseType
        ) PURE;

    STDMETHOD(GetHashCode)(
        THIS_
        _Out_ ULONG* hashCode
        ) PURE;

    STDMETHOD(GetIntrinsicType)(
        THIS_
        _Out_opt_ IntrinsicKind *intrinsicKind,
        _Out_opt_ VARTYPE *carrierType
        ) PURE;

    //*************************************************
    // Bitfield Information:
    //
    // The following methods only apply to types which are bitfields:
    //

    STDMETHOD(GetBitField)(
        THIS_
        _Out_ ULONG* lsbOfField,
        _Out_ ULONG* lengthOfField
        ) PURE;

    //*************************************************
    // Pointer Information (GetKind returns TypePointer):
    //
    // The following methods only apply to types which are pointers (or create
    // derivative types which are pointers)
    //

    STDMETHOD(GetPointerKind)(
        THIS_
        _Out_ PointerKind* pointerKind
        ) PURE;

    STDMETHOD(GetMemberType)(
        THIS_
        _Out_ IDebugHostType** memberType
        ) PURE;

    STDMETHOD(CreatePointerTo)(
        THIS_
        _In_ PointerKind kind,
        _COM_Outptr_ IDebugHostType** newType
        ) PURE;

    //*************************************************
    // Array Information (GetKind returns TypeArray):
    //
    // The following methods only apply to types which are arrays (or create
    // derivative types which are arrays)
    //

    STDMETHOD(GetArrayDimensionality)(
        THIS_
        _Out_ ULONG64* arrayDimensionality
        ) PURE;

    STDMETHOD(GetArrayDimensions)(
        THIS_
        _In_ ULONG64 dimensions,
        _Out_writes_(dimensions) ArrayDimension *pDimensions
        ) PURE;

    STDMETHOD(CreateArrayOf)(
        THIS_
        _In_ ULONG64 dimensions,
        _In_reads_(dimensions) ArrayDimension *pDimensions,
        _COM_Outptr_ IDebugHostType** newType
        ) PURE;

    //*************************************************
    // Function Information (GetKind returns TypeFunction):
    //
    // The following methods only apply to types which are functions (or create
    // derivative types which are functions)
    //

    STDMETHOD(GetFunctionCallingConvention)(
        THIS_
        _Out_ CallingConventionKind* conventionKind
        ) PURE;

    STDMETHOD(GetFunctionReturnType)(
        THIS_
        _COM_Outptr_ IDebugHostType** returnType
        ) PURE;

    STDMETHOD(GetFunctionParameterTypeCount)(
        THIS_
        _Out_ ULONG64* count
        ) PURE;

    STDMETHOD(GetFunctionParameterTypeAt)(
        THIS_
        _In_ ULONG64 i,
        _Out_ IDebugHostType** parameterType
        ) PURE;

    //*************************************************
    // Template Information:
    //
    // The following methods only apply to types which are templates or generics (or create
    // derivative types which are templates or generics)
    //

    STDMETHOD(IsGeneric)(
        THIS_
        _Out_ bool* isGeneric
        ) PURE;

    STDMETHOD(GetGenericArgumentCount)(
        THIS_
        _Out_ ULONG64* argCount
        ) PURE;

    STDMETHOD(GetGenericArgumentAt)(
        THIS_
        _In_ ULONG64 i,
        _Out_ IDebugHostSymbol** argument
        ) PURE;

    //**************************************************************************
    // IDebugHostType2:
    //
    // The following methods are only available in the v2 IDebugHostType interface.

    //*************************************************
    // Typedef Information:
    //
    // Typedef types are, for the vast majority of calls, indistinguishable from the types they are definitions
    // against.  In the event that the true typedef information needs to be known, the following APIs
    // apply.
    //

    // IsTypedef():
    //
    // Returns whether the type is a typedef for another type or not.
    //
    STDMETHOD(IsTypedef)(
        THIS_
        _Out_ bool* isTypedef
        ) PURE;

    // GetTypedefBaseType():
    //
    // If IsTypedef returns true, this will return the type that a typedef refers to.  Note that the base
    // type may be another typedef.  It is entirely possible to have a chain of definitions.  If the caller
    // wishes to acquire the first non-typedef type of a chain, GetTypedefFinalBaseType is the appropriate call.
    //
    STDMETHOD(GetTypedefBaseType)(
        THIS_
        _Out_ IDebugHostType2** baseType
        ) PURE;

    // GetTypedefFinalBaseType():
    //
    // If IsTypedef returns true, this will return the bottom-most type of a typedef chain.  That is, the first
    // type in the chain which is NOT a typedef.  This is the type which is implicitly used for most
    // non-typedef IDebugHostType::* methods on a typedef type.
    //
    STDMETHOD(GetTypedefFinalBaseType)(
        THIS_
        _Out_ IDebugHostType2** finalBaseType
        ) PURE;

    //*************************************************
    // Extended Function Information:
    //
    // The following methods indicate additional information about function types that is only available through
    // IDebugHostType2.
    //

    // GetFunctionVarArgsKind():
    //
    // Indicates whether and what kind of variable arguments a function takes.
    //
    STDMETHOD(GetFunctionVarArgsKind)(
        THIS_
        _Out_ VarArgsKind* varArgsKind
        ) PURE;

    // GetFunctionInstancePointerType():
    //
    // Indicates what the type of the instance ("this") pointer passed to the function is.  This method will fail
    // if the function is not an instance method on a class.
    //
    STDMETHOD(GetFunctionInstancePointerType)(
        THIS_
        _Out_ IDebugHostType2** instancePointerType
        ) PURE;

    //**************************************************************************
    // IDebugHostType3:
    //
    // The following methods are only available in the IDebugHostTyp3 interface.

    // GetContainingType():
    //
    // Returns the type of the containing parent (containing this symbol)
    //
    STDMETHOD(GetContainingType)(
        THIS_
        _Out_ IDebugHostType3** containingParentType
        ) PURE;

    //**************************************************************************
    // IDebugHostType4:
    //
    // The following methods are only available in the IDebugHostType4 interface.
    //

    // GetExtendedArrayHeaderSize():
    //
    // If the array has a header including layout information, this returns the size of such header.  This
    // is the offset from the start of the array to the first element in the array as described by the
    // extended array dimensions.
    //
    STDMETHOD(GetExtendedArrayHeaderSize)(
        THIS_
        _Out_ ULONG64* headerSize
        ) PURE;

    // GetExtendedArrayDimensions():
    //
    // Fills in information about each dimension of the array including its lower bound, length, and stride.
    // This method should not be called on TypeExtendedArray.  Methods in IDebugHostType3 should be utilized.
    //
    STDMETHOD(GetExtendedArrayDimensions)(
        THIS_
        _In_ ULONG64 dimensions,
        _Out_writes_(dimensions) ExtendedArrayDimension *pDimensions
        ) PURE;

    //*************************************************
    // UDT Information (GetKind returns TypeUDT):
    //
    // The following methods only apply to types which are user-defined types:
    //

    // GetUDTKind():
    //
    // Returns a value indicating whether the UDT is a struct, union, class, interface or tagged union.
    //
    STDMETHOD(GetUDTKind)(
        THIS_
        _Out_ UDTKind* udtKind
        ) PURE;
};

//
// IDebugHostType5:
//
// Additional type information capabilities provided in addition to those of IDebugHostType.
//
#undef INTERFACE
#define INTERFACE IDebugHostType5
DECLARE_INTERFACE_(IDebugHostType5, IDebugHostType4)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    //*************************************************
    // IDebugHostType:

    STDMETHOD(GetTypeKind)(
        THIS_
        _Out_ TypeKind *kind
        ) PURE;

    STDMETHOD(GetSize)(
        THIS_
        _Out_ ULONG64* size
        ) PURE;

    STDMETHOD(GetBaseType)(
        THIS_
        _Out_ IDebugHostType** baseType
        ) PURE;

    STDMETHOD(GetHashCode)(
        THIS_
        _Out_ ULONG* hashCode
        ) PURE;

    STDMETHOD(GetIntrinsicType)(
        THIS_
        _Out_opt_ IntrinsicKind *intrinsicKind,
        _Out_opt_ VARTYPE *carrierType
        ) PURE;

    //*************************************************
    // Bitfield Information:
    //
    // The following methods only apply to types which are bitfields:
    //

    STDMETHOD(GetBitField)(
        THIS_
        _Out_ ULONG* lsbOfField,
        _Out_ ULONG* lengthOfField
        ) PURE;

    //*************************************************
    // Pointer Information (GetKind returns TypePointer):
    //
    // The following methods only apply to types which are pointers (or create
    // derivative types which are pointers)
    //

    STDMETHOD(GetPointerKind)(
        THIS_
        _Out_ PointerKind* pointerKind
        ) PURE;

    STDMETHOD(GetMemberType)(
        THIS_
        _Out_ IDebugHostType** memberType
        ) PURE;

    STDMETHOD(CreatePointerTo)(
        THIS_
        _In_ PointerKind kind,
        _COM_Outptr_ IDebugHostType** newType
        ) PURE;

    //*************************************************
    // Array Information (GetKind returns TypeArray):
    //
    // The following methods only apply to types which are arrays (or create
    // derivative types which are arrays)
    //

    STDMETHOD(GetArrayDimensionality)(
        THIS_
        _Out_ ULONG64* arrayDimensionality
        ) PURE;

    STDMETHOD(GetArrayDimensions)(
        THIS_
        _In_ ULONG64 dimensions,
        _Out_writes_(dimensions) ArrayDimension *pDimensions
        ) PURE;

    STDMETHOD(CreateArrayOf)(
        THIS_
        _In_ ULONG64 dimensions,
        _In_reads_(dimensions) ArrayDimension *pDimensions,
        _COM_Outptr_ IDebugHostType** newType
        ) PURE;

    //*************************************************
    // Function Information (GetKind returns TypeFunction):
    //
    // The following methods only apply to types which are functions (or create
    // derivative types which are functions)
    //

    STDMETHOD(GetFunctionCallingConvention)(
        THIS_
        _Out_ CallingConventionKind* conventionKind
        ) PURE;

    STDMETHOD(GetFunctionReturnType)(
        THIS_
        _COM_Outptr_ IDebugHostType** returnType
        ) PURE;

    STDMETHOD(GetFunctionParameterTypeCount)(
        THIS_
        _Out_ ULONG64* count
        ) PURE;

    STDMETHOD(GetFunctionParameterTypeAt)(
        THIS_
        _In_ ULONG64 i,
        _Out_ IDebugHostType** parameterType
        ) PURE;

    //*************************************************
    // Template Information:
    //
    // The following methods only apply to types which are templates or generics (or create
    // derivative types which are templates or generics)
    //

    STDMETHOD(IsGeneric)(
        THIS_
        _Out_ bool* isGeneric
        ) PURE;

    STDMETHOD(GetGenericArgumentCount)(
        THIS_
        _Out_ ULONG64* argCount
        ) PURE;

    STDMETHOD(GetGenericArgumentAt)(
        THIS_
        _In_ ULONG64 i,
        _Out_ IDebugHostSymbol** argument
        ) PURE;

    //**************************************************************************
    // IDebugHostType2:
    //
    // The following methods are only available in the v2 IDebugHostType interface.

    //*************************************************
    // Typedef Information:
    //
    // Typedef types are, for the vast majority of calls, indistinguishable from the types they are definitions
    // against.  In the event that the true typedef information needs to be known, the following APIs
    // apply.
    //

    // IsTypedef():
    //
    // Returns whether the type is a typedef for another type or not.
    //
    STDMETHOD(IsTypedef)(
        THIS_
        _Out_ bool* isTypedef
        ) PURE;

    // GetTypedefBaseType():
    //
    // If IsTypedef returns true, this will return the type that a typedef refers to.  Note that the base
    // type may be another typedef.  It is entirely possible to have a chain of definitions.  If the caller
    // wishes to acquire the first non-typedef type of a chain, GetTypedefFinalBaseType is the appropriate call.
    //
    STDMETHOD(GetTypedefBaseType)(
        THIS_
        _Out_ IDebugHostType2** baseType
        ) PURE;

    // GetTypedefFinalBaseType():
    //
    // If IsTypedef returns true, this will return the bottom-most type of a typedef chain.  That is, the first
    // type in the chain which is NOT a typedef.  This is the type which is implicitly used for most
    // non-typedef IDebugHostType::* methods on a typedef type.
    //
    STDMETHOD(GetTypedefFinalBaseType)(
        THIS_
        _Out_ IDebugHostType2** finalBaseType
        ) PURE;

    //*************************************************
    // Extended Function Information:
    //
    // The following methods indicate additional information about function types that is only available through
    // IDebugHostType2.
    //

    // GetFunctionVarArgsKind():
    //
    // Indicates whether and what kind of variable arguments a function takes.
    //
    STDMETHOD(GetFunctionVarArgsKind)(
        THIS_
        _Out_ VarArgsKind* varArgsKind
        ) PURE;

    // GetFunctionInstancePointerType():
    //
    // Indicates what the type of the instance ("this") pointer passed to the function is.  This method will fail
    // if the function is not an instance method on a class.
    //
    STDMETHOD(GetFunctionInstancePointerType)(
        THIS_
        _Out_ IDebugHostType2** instancePointerType
        ) PURE;

    //**************************************************************************
    // IDebugHostType3:
    //
    // The following methods are only available in the IDebugHostTyp3 interface.

    // GetContainingType():
    //
    // Returns the type of the containing parent (containing this symbol)
    //
    STDMETHOD(GetContainingType)(
        THIS_
        _Out_ IDebugHostType3** containingParentType
        ) PURE;

    //**************************************************************************
    // IDebugHostType4:
    //
    // The following methods are only available in the IDebugHostType4 interface.
    //

    // GetExtendedArrayHeaderSize():
    //
    // If the array has a header including layout information, this returns the size of such header.  This
    // is the offset from the start of the array to the first element in the array as described by the
    // extended array dimensions.
    //
    STDMETHOD(GetExtendedArrayHeaderSize)(
        THIS_
        _Out_ ULONG64* headerSize
        ) PURE;

    // GetExtendedArrayDimensions():
    //
    // Fills in information about each dimension of the array including its lower bound, length, and stride.
    // This method should not be called on TypeExtendedArray.  Methods in IDebugHostType3 should be utilized.
    //
    STDMETHOD(GetExtendedArrayDimensions)(
        THIS_
        _In_ ULONG64 dimensions,
        _Out_writes_(dimensions) ExtendedArrayDimension *pDimensions
        ) PURE;

    //*************************************************
    // UDT Information (GetKind returns TypeUDT):
    //
    // The following methods only apply to types which are user-defined types:
    //

    // GetUDTKind():
    //
    // Returns a value indicating whether the UDT is a struct, union, class, or interface.
    //
    STDMETHOD(GetUDTKind)(
        THIS_
        _Out_ UDTKind* udtKind
        ) PURE;

    //**************************************************************************
    // IDebugHostType5:
    //
    // The following methods are only available in the IDebugHostType5 interface.
    //

    // IsBaseTypeOf():
    //
    // Returns whether this type is a base type of another type.
    //
    STDMETHOD(IsBaseTypeOf)(
        THIS_
        _In_ IDebugHostType* pOtherType,
        _Out_ bool* pIsBase
        ) PURE;
};

//
// IDebugHostType6:
//
// Additional type information capabilities provided in addition to those of IDebugHostType.
//
#undef INTERFACE
#define INTERFACE IDebugHostType6
DECLARE_INTERFACE_(IDebugHostType6, IDebugHostType5)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    //*************************************************
    // IDebugHostType:

    STDMETHOD(GetTypeKind)(
        THIS_
        _Out_ TypeKind *kind
        ) PURE;

    STDMETHOD(GetSize)(
        THIS_
        _Out_ ULONG64* size
        ) PURE;

    STDMETHOD(GetBaseType)(
        THIS_
        _Out_ IDebugHostType** baseType
        ) PURE;

    STDMETHOD(GetHashCode)(
        THIS_
        _Out_ ULONG* hashCode
        ) PURE;

    STDMETHOD(GetIntrinsicType)(
        THIS_
        _Out_opt_ IntrinsicKind *intrinsicKind,
        _Out_opt_ VARTYPE *carrierType
        ) PURE;

    //*************************************************
    // Bitfield Information:
    //
    // The following methods only apply to types which are bitfields:
    //

    STDMETHOD(GetBitField)(
        THIS_
        _Out_ ULONG* lsbOfField,
        _Out_ ULONG* lengthOfField
        ) PURE;

    //*************************************************
    // Pointer Information (GetKind returns TypePointer):
    //
    // The following methods only apply to types which are pointers (or create
    // derivative types which are pointers)
    //

    STDMETHOD(GetPointerKind)(
        THIS_
        _Out_ PointerKind* pointerKind
        ) PURE;

    STDMETHOD(GetMemberType)(
        THIS_
        _Out_ IDebugHostType** memberType
        ) PURE;

    STDMETHOD(CreatePointerTo)(
        THIS_
        _In_ PointerKind kind,
        _COM_Outptr_ IDebugHostType** newType
        ) PURE;

    //*************************************************
    // Array Information (GetKind returns TypeArray):
    //
    // The following methods only apply to types which are arrays (or create
    // derivative types which are arrays)
    //

    STDMETHOD(GetArrayDimensionality)(
        THIS_
        _Out_ ULONG64* arrayDimensionality
        ) PURE;

    STDMETHOD(GetArrayDimensions)(
        THIS_
        _In_ ULONG64 dimensions,
        _Out_writes_(dimensions) ArrayDimension *pDimensions
        ) PURE;

    STDMETHOD(CreateArrayOf)(
        THIS_
        _In_ ULONG64 dimensions,
        _In_reads_(dimensions) ArrayDimension *pDimensions,
        _COM_Outptr_ IDebugHostType** newType
        ) PURE;

    //*************************************************
    // Function Information (GetKind returns TypeFunction):
    //
    // The following methods only apply to types which are functions (or create
    // derivative types which are functions)
    //

    STDMETHOD(GetFunctionCallingConvention)(
        THIS_
        _Out_ CallingConventionKind* conventionKind
        ) PURE;

    STDMETHOD(GetFunctionReturnType)(
        THIS_
        _COM_Outptr_ IDebugHostType** returnType
        ) PURE;

    STDMETHOD(GetFunctionParameterTypeCount)(
        THIS_
        _Out_ ULONG64* count
        ) PURE;

    STDMETHOD(GetFunctionParameterTypeAt)(
        THIS_
        _In_ ULONG64 i,
        _Out_ IDebugHostType** parameterType
        ) PURE;

    //*************************************************
    // Template Information:
    //
    // The following methods only apply to types which are templates or generics (or create
    // derivative types which are templates or generics)
    //

    STDMETHOD(IsGeneric)(
        THIS_
        _Out_ bool* isGeneric
        ) PURE;

    STDMETHOD(GetGenericArgumentCount)(
        THIS_
        _Out_ ULONG64* argCount
        ) PURE;

    STDMETHOD(GetGenericArgumentAt)(
        THIS_
        _In_ ULONG64 i,
        _Out_ IDebugHostSymbol** argument
        ) PURE;

    //**************************************************************************
    // IDebugHostType2:
    //
    // The following methods are only available in the v2 IDebugHostType interface.

    //*************************************************
    // Typedef Information:
    //
    // Typedef types are, for the vast majority of calls, indistinguishable from the types they are definitions
    // against.  In the event that the true typedef information needs to be known, the following APIs
    // apply.
    //

    // IsTypedef():
    //
    // Returns whether the type is a typedef for another type or not.
    //
    STDMETHOD(IsTypedef)(
        THIS_
        _Out_ bool* isTypedef
        ) PURE;

    // GetTypedefBaseType():
    //
    // If IsTypedef returns true, this will return the type that a typedef refers to.  Note that the base
    // type may be another typedef.  It is entirely possible to have a chain of definitions.  If the caller
    // wishes to acquire the first non-typedef type of a chain, GetTypedefFinalBaseType is the appropriate call.
    //
    STDMETHOD(GetTypedefBaseType)(
        THIS_
        _Out_ IDebugHostType2** baseType
        ) PURE;

    // GetTypedefFinalBaseType():
    //
    // If IsTypedef returns true, this will return the bottom-most type of a typedef chain.  That is, the first
    // type in the chain which is NOT a typedef.  This is the type which is implicitly used for most
    // non-typedef IDebugHostType::* methods on a typedef type.
    //
    STDMETHOD(GetTypedefFinalBaseType)(
        THIS_
        _Out_ IDebugHostType2** finalBaseType
        ) PURE;

    //*************************************************
    // Extended Function Information:
    //
    // The following methods indicate additional information about function types that is only available through
    // IDebugHostType2.
    //

    // GetFunctionVarArgsKind():
    //
    // Indicates whether and what kind of variable arguments a function takes.
    //
    STDMETHOD(GetFunctionVarArgsKind)(
        THIS_
        _Out_ VarArgsKind* varArgsKind
        ) PURE;

    // GetFunctionInstancePointerType():
    //
    // Indicates what the type of the instance ("this") pointer passed to the function is.  This method will fail
    // if the function is not an instance method on a class.
    //
    STDMETHOD(GetFunctionInstancePointerType)(
        THIS_
        _Out_ IDebugHostType2** instancePointerType
        ) PURE;

    //**************************************************************************
    // IDebugHostType3:
    //
    // The following methods are only available in the IDebugHostTyp3 interface.

    // GetContainingType():
    //
    // Returns the type of the containing parent (containing this symbol)
    //
    STDMETHOD(GetContainingType)(
        THIS_
        _Out_ IDebugHostType3** containingParentType
        ) PURE;

    //**************************************************************************
    // IDebugHostType4:
    //
    // The following methods are only available in the IDebugHostType4 interface.
    //

    // GetExtendedArrayHeaderSize():
    //
    // If the array has a header including layout information, this returns the size of such header.  This
    // is the offset from the start of the array to the first element in the array as described by the
    // extended array dimensions.
    //
    STDMETHOD(GetExtendedArrayHeaderSize)(
        THIS_
        _Out_ ULONG64* headerSize
        ) PURE;

    // GetExtendedArrayDimensions():
    //
    // Fills in information about each dimension of the array including its lower bound, length, and stride.
    // This method should not be called on TypeExtendedArray.  Methods in IDebugHostType3 should be utilized.
    //
    STDMETHOD(GetExtendedArrayDimensions)(
        THIS_
        _In_ ULONG64 dimensions,
        _Out_writes_(dimensions) ExtendedArrayDimension *pDimensions
        ) PURE;

    //*************************************************
    // UDT Information (GetKind returns TypeUDT):
    //
    // The following methods only apply to types which are user-defined types:
    //

    // GetUDTKind():
    //
    // Returns a value indicating whether the UDT is a struct, union, class, or interface.
    //
    STDMETHOD(GetUDTKind)(
        THIS_
        _Out_ UDTKind* udtKind
        ) PURE;

    //**************************************************************************
    // IDebugHostType5:
    //
    // The following methods are only available in the IDebugHostType5 interface.
    //

    // IsBaseTypeOf():
    //
    // Returns whether this type is a base type of another type.
    //
    STDMETHOD(IsBaseTypeOf)(
        THIS_
        _In_ IDebugHostType* pOtherType,
        _Out_ bool* pIsBase
        ) PURE;

    //**************************************************************************
    // IDebugHostType6:
    //
    // The following methods are only available in the IDebugHostType6 interface.
    //

    // GetTaggedUnionTag():
    //
    // For cases within a tagged union type, this returns the type and offset of the tag
    // as well as any mask value that should be applied to the tag before comparison.
    //
    STDMETHOD(GetTaggedUnionTag)(
        THIS_
        _Out_ IDebugHostType** pTagType,
        _Out_ ULONG* pTagOffset,
        _Out_ VARIANT* pTagMask
        ) PURE;

    // GetTaggedUnionTagRanges():
    //
    // For cases within a tagged union type, this returns an enumerator over the tag ranges.
    //
    STDMETHOD(GetTaggedUnionTagRanges)(
        THIS_
        _Out_ IDebugHostTaggedUnionRangeEnumerator** pTagRangeEnumerator
        ) PURE;

    // UpcastToTaggedUnionType():
    //
    // For cases within a tagged union type, this will return a type that is conceptually
    // the same as the tagged union type but narrowed to this case such that enumerating
    // children of the type will return children of this case.
    STDMETHOD(UpcastToTaggedUnionType)(
        THIS_
        _In_ IDebugHostType* pTaggedUnionType,
        _Out_ IDebugHostType** pUpcastedCaseType
        ) PURE;
};

//
// IDebugHostTaggedUnionRangeEnumerator:
//
// This enumerates a set of tag ranges for a tagged union case. This can be acquired by calling
// IDebugHostType6::GetTaggedUnionTagRanges() on a tagged union case type.
//
#undef INTERFACE
#define INTERFACE IDebugHostTaggedUnionRangeEnumerator
DECLARE_INTERFACE_(IDebugHostTaggedUnionRangeEnumerator, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostTaggedUnionRangeEnumerator:

    // Reset():
    //
    // Resets the enumerator to its initial state.  A subsequent GetNext call will return
    // the first low/high tag pair in the set in enumerator order.
    //
    STDMETHOD(Reset)(
        THIS
        ) PURE;

    // GetNext():
    //
    // Moves the iterator forward and fetches the next low/high tag pair in the set.
    //
    // E_BOUNDS will be returned when the enumerator hits the end of the set.
    //
    STDMETHOD(GetNext)(
        THIS_
        _Out_ VARIANT* pLow,
        _Out_ VARIANT* pHigh
        ) PURE;

    // GetCount():
    //
    // Returns the total number of tag values (not pairs) that will be returned from GetNext().
    //
    STDMETHOD(GetCount)(
        THIS_
        _Out_ ULONG* pCount
    ) PURE;
};


//
// IDebugHostConstant:
//
// A specialization of IDebugHostSymbol which represents a constant symbol.
// Such may be returned as the literal value for some symbol.
//
#undef INTERFACE
#define INTERFACE IDebugHostConstant
DECLARE_INTERFACE_(IDebugHostConstant, IDebugHostSymbol)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    //*************************************************
    // IDebugHostConstant:

    // GetValue():
    //
    // Returns the value of the constant in a VARIANT data structure.
    //
    STDMETHOD(GetValue)(
        THIS_
        _Out_ VARIANT* value
        ) PURE;

};

//
// IDebugHostField:
//
// A specialization of IDebugHostSymbol which represents a field of a class or struct.
//
#undef INTERFACE
#define INTERFACE IDebugHostField
DECLARE_INTERFACE_(IDebugHostField, IDebugHostSymbol)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    //*************************************************
    // IDebugHostField:

    // GetLocationKind():
    //
    // Returns the kind of location the field is at.  Such location may be static, member, constant,
    // etc...
    //
    STDMETHOD(GetLocationKind)(
        THIS_
        _Out_ LocationKind *locationKind
        ) PURE;

    // GetOffset():
    //
    // If the location kind indicates that the field is a member, this returns the offset into the class
    // or struct of which the field is a member.
    //
    STDMETHOD(GetOffset)(
        THIS_
        _Out_ ULONG64* offset
        ) PURE;

    // GetLocation():
    //
    // If the location kind indicates that the field is static, this returns the location of the field in
    // the address space of the debug target.
    //
    STDMETHOD(GetLocation)(
        THIS_
        _Out_ Location* location
        ) PURE;

    // GetValue():
    //
    // If the location kind indicates that the field is constant, this returns the constant value of the field.
    //
    STDMETHOD(GetValue)(
        THIS_
        _Out_ VARIANT* value
        ) PURE;

};

//
// IDebugHostField:
//
// A specialization of IDebugHostSymbol which represents a field of a class or struct.
//
#undef INTERFACE
#define INTERFACE IDebugHostField2
DECLARE_INTERFACE_(IDebugHostField2, IDebugHostField)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    //*************************************************
    // IDebugHostField:

    // GetLocationKind():
    //
    // Returns the kind of location the field is at.  Such location may be static, member, constant,
    // etc...
    //
    STDMETHOD(GetLocationKind)(
        THIS_
        _Out_ LocationKind *locationKind
        ) PURE;

    // GetOffset():
    //
    // If the location kind indicates that the field is a member, this returns the offset into the class
    // or struct of which the field is a member.
    //
    STDMETHOD(GetOffset)(
        THIS_
        _Out_ ULONG64* offset
        ) PURE;

    // GetLocation():
    //
    // If the location kind indicates that the field is static, this returns the location of the field in
    // the address space of the debug target.
    //
    STDMETHOD(GetLocation)(
        THIS_
        _Out_ Location* location
        ) PURE;

    // GetValue():
    //
    // If the location kind indicates that the field is constant, this returns the constant value of the field.
    //
    STDMETHOD(GetValue)(
        THIS_
        _Out_ VARIANT* value
        ) PURE;

    //**************************************************************************
    // IDebugHostField2:
    //
    // The following methods are only available in the IDebugHostField2 interface.

    // GetContainingType():
    //
    // Returns the type of the containing parent (containing this symbol)
    //
    STDMETHOD(GetContainingType)(
        THIS_
        _Out_ IDebugHostType3** containingParentType
        ) PURE;
};

//
// IDebugHostData:
//
// A specialization of IDebugHostSymbol which represents data which is unattached to a class or struct
// (e.g.: a global variable)
//
#undef INTERFACE
#define INTERFACE IDebugHostData
DECLARE_INTERFACE_(IDebugHostData, IDebugHostSymbol)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    //*************************************************
    // IDebugHostData:

    // GetLocationKind():
    //
    // Returns the kind of location the data is at.  Such location may be static, constant, etc...
    //
    STDMETHOD(GetLocationKind)(
        THIS_
        _Out_ LocationKind *locationKind
        ) PURE;

    // GetLocation():
    //
    // If the location kind indicates that the data is static, this returns the location of the data in
    // the address space of the debug target.
    //
    STDMETHOD(GetLocation)(
        THIS_
        _Out_ Location* location
        ) PURE;

    // GetValue():
    //
    // If the location kind indicates that the data is constant, this returns the constant value of the data.
    //
    STDMETHOD(GetValue)(
        THIS_
        _Out_ VARIANT* value
        ) PURE;
};

// IDebugHostPublic:
//
// A specialization of IDebugHostSymbol which represents a public symbol entry.
//
#undef INTERFACE
#define INTERFACE IDebugHostPublic
DECLARE_INTERFACE_(IDebugHostPublic, IDebugHostSymbol)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    //*************************************************
    // IDebugHostPublic:

    // GetLocationKind():
    //
    // Returns the kind of location the data is at.  Such location may be static, constant, etc...
    //
    STDMETHOD(GetLocationKind)(
        THIS_
        _Out_ LocationKind *locationKind
        ) PURE;

    // GetLocation():
    //
    // If the location kind indicates that the data is static, this returns the location of the data in
    // the address space of the debug target.
    //
    STDMETHOD(GetLocation)(
        THIS_
        _Out_ Location* location
        ) PURE;

};

//
// IDebugHostBaseClass:
//
// A specialization of IDebugHostSymbol which represents a base class.
//
#undef INTERFACE
#define INTERFACE IDebugHostBaseClass
DECLARE_INTERFACE_(IDebugHostBaseClass, IDebugHostSymbol)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    //*************************************************
    // IDebugHostBaseClass:

    // GetOffset():
    //
    // This returns the offset of the base class within the class.
    //
    STDMETHOD(GetOffset)(
        THIS_
        _Out_ ULONG64* offset
        ) PURE;

};

//
// IDebugHostBaseClass2:
//
// An extended specialization of IDebugHostSymbol which represents a base class.
//
#undef INTERFACE
#define INTERFACE IDebugHostBaseClass2
DECLARE_INTERFACE_(IDebugHostBaseClass2, IDebugHostBaseClass)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    //*************************************************
    // IDebugHostBaseClass:

    STDMETHOD(GetOffset)(
        THIS_
        _Out_ ULONG64* offset
        ) PURE;

    //*************************************************
    // IDebugHostBaseClass2:
    //

    // IsVirtual():
    //
    // An indication of whether this particular base class is a virtual base class or not.  If
    // a given base class is virtual (and it's location cannot be described with a simple offset), GetOffset 
    // may fail with E_NOT_SET and a more complex location may be returned via methods on IDebugHostBaseClass2.
    //
    STDMETHOD(IsVirtual)(
        THIS_
        _Out_ bool *pIsVirtual
        ) PURE;

    // GetVirtualBaseOffsetLocation():
    // 
    // Gets the location of the "offset" of the base class relative to the parent class.  In essence, the
    // location of the vtbl/vbtbl is (<object> + *pTableOffset) and the location of the offset within that table
    // is given as (v[b]tbl + pSlotOffset).
    // 
    // The size of the slot is given as *pSlotSize and *pSlotIsSigned indicates whether or not such offset read
    // from that slot  should be considered a signed or unsigned value.
    //
    STDMETHOD(GetVirtualBaseOffsetLocation)(
        THIS_
        _Out_ LONG64* pTableOffset,
        _Out_ LONG64* pSlotOffset,
        _Out_ ULONG64 *pSlotSize,
        _Out_ bool *pSlotIsSigned
        ) PURE;
};

//
// IDebugHostSymbols:
//
// The core symbols interface which a debug host presents.  This interface can be QI'd from
// IDebugHost in order to access global symbols, the module list of the debug target, type
// signatures, etc...
//
#undef INTERFACE
#define INTERFACE IDebugHostSymbols
DECLARE_INTERFACE_(IDebugHostSymbols, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbols:

    // CreateModuleSignature()
    //
    // Creates a module signature for the given module name and version range.
    //
    STDMETHOD(CreateModuleSignature)(
        THIS_
        _In_z_ PCWSTR pwszModuleName,
        _In_opt_z_ PCWSTR pwszMinVersion,
        _In_opt_z_ PCWSTR pwszMaxVersion,
        _Out_ IDebugHostModuleSignature** ppModuleSignature
        ) PURE;

    // CreateTypeSignature():
    //
    // Creates a type signature object.  A type signature is a generalization of a type which many
    // specific instances can match.  The format of type signatures is specific to the host and language
    // to which they refer.
    //
    // For current hosts and C/C++, a type signature here is equivalent to a NatVis type specification.
    // It is the name of a type with wildcards allowed for template arguments.
    //
    // If a specific IDebugHostModule is passed in the "module" argument, the type signature only applies
    // to types within that specific module as defined by the host.
    //
    STDMETHOD(CreateTypeSignature)(
        THIS_
        _In_z_ PCWSTR signatureSpecification,
        _In_opt_ IDebugHostModule* module,
        _Out_ IDebugHostTypeSignature** typeSignature
        ) PURE;

    // CreateTypeSignatureForModuleRange():
    //
    // Creates a type signature object.  A type signature is a generalization of a type which many
    // specific instances can match.  The format of type signatures is specific to the host and language
    // to which they refer.
    //
    // For current hosts and C/C++, a type signature here is equivalent to a NatVis type specification.
    // It is the name of a type with wildcards allowed for template arguments.
    //
    // This method allows the type signature to apply to modules with a particular name and in a range
    // of versions as defined by an "x", "x.y", "x.y.z", or "x.y.z.a" version string.
    //
    // moduleName only        - The type signature is restricted to modules with the specified name
    // moduleName, minVersion - The type signature is restricted to modules with the specified name of at least the specified version
    // moduleName, maxVersion - The type signature is restricted to modules with the specified name of at most the specified version
    // moduleName, minVersion,
    //             maxVersion - The type signature is restricted to modules with the specified name within the range of supplied version numbers
    //
    STDMETHOD(CreateTypeSignatureForModuleRange)(
        THIS_
        _In_z_ PCWSTR signatureSpecification,
        _In_z_ PCWSTR moduleName,
        _In_opt_z_ PCWSTR minVersion,
        _In_opt_z_ PCWSTR maxVersion,
        _Out_ IDebugHostTypeSignature** typeSignature
        ) PURE;

    // EnumerateModules():
    //
    // Returns an enumerator which enumerates the module list for a given context.  The context supplied
    // here may be acquired from IDebugHost::GetCurrentContext or any of the various interfaces which have a
    // GetContext(...) method.
    //
    STDMETHOD(EnumerateModules)(
        THIS_
        _In_ IDebugHostContext* context,
        _COM_Outptr_ IDebugHostSymbolEnumerator** moduleEnum
        ) PURE;

    // FindModuleByName():
    //
    // Finds a particular module by name within the given context.  If such module cannot be found, an error occurs.
    // It is legal to ask for the module with or without the extension.
    //
    STDMETHOD(FindModuleByName)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_z_ PCWSTR moduleName,
        _COM_Outptr_ IDebugHostModule **module
        ) PURE;

    // FindModuleByLocation():
    //
    // Given a location within the address space of the debug target as defined by the given context, this returns
    // the module to which that location belongs.  If that location is not within the address range of a mapped
    // module, an error is returned.
    //
    STDMETHOD(FindModuleByLocation)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location moduleLocation,
        _COM_Outptr_ IDebugHostModule **module
        ) PURE;

    // GetMostDerivedObject():
    //
    // For an object of a given type at a specified location within the address space of the debug target as defined
    // by the given context, this will attempt to perform runtime type analysis to determine the runtime type of
    // the object (as opposed to the passed static type).  If such a runtime type can be found, the location of the
    // runtime (most derived) type will be returned as well as its type.
    //
    // Note that analysis takes place at the type system level only and has nothing to do with the
    // IPreferredRuntimeTypeConcept concept.
    //
    // Note that this method may fail for a variety of reasons.  It may also return the same address and type which
    // was input.
    //
    STDMETHOD(GetMostDerivedObject)(
        THIS_
        _In_opt_ IDebugHostContext *pContext,
        _In_ Location location,
        _In_ IDebugHostType* objectType,
        _Out_ Location* derivedLocation,
        _Out_ IDebugHostType** derivedType
        ) PURE;
};

//
// IDebugHostSymbols2:
//
// The core symbols interface which a debug host presents.  This interface can be QI'd from
// IDebugHost in order to access global symbols, the module list of the debug target, type
// signatures, etc...
//
#undef INTERFACE
#define INTERFACE IDebugHostSymbols2
DECLARE_INTERFACE_(IDebugHostSymbols2, IDebugHostSymbols)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbols:

    STDMETHOD(CreateModuleSignature)(
        THIS_
        _In_z_ PCWSTR pwszModuleName,
        _In_opt_z_ PCWSTR pwszMinVersion,
        _In_opt_z_ PCWSTR pwszMaxVersion,
        _Out_ IDebugHostModuleSignature** ppModuleSignature
        ) PURE;

    STDMETHOD(CreateTypeSignature)(
        THIS_
        _In_z_ PCWSTR signatureSpecification,
        _In_opt_ IDebugHostModule* module,
        _Out_ IDebugHostTypeSignature** typeSignature
        ) PURE;

    STDMETHOD(CreateTypeSignatureForModuleRange)(
        THIS_
        _In_z_ PCWSTR signatureSpecification,
        _In_z_ PCWSTR moduleName,
        _In_opt_z_ PCWSTR minVersion,
        _In_opt_z_ PCWSTR maxVersion,
        _Out_ IDebugHostTypeSignature** typeSignature
        ) PURE;

    STDMETHOD(EnumerateModules)(
        THIS_
        _In_ IDebugHostContext* context,
        _COM_Outptr_ IDebugHostSymbolEnumerator** moduleEnum
        ) PURE;

    STDMETHOD(FindModuleByName)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_z_ PCWSTR moduleName,
        _COM_Outptr_ IDebugHostModule **module
        ) PURE;

    STDMETHOD(FindModuleByLocation)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location moduleLocation,
        _COM_Outptr_ IDebugHostModule **module
        ) PURE;

    STDMETHOD(GetMostDerivedObject)(
        THIS_
        _In_opt_ IDebugHostContext *pContext,
        _In_ Location location,
        _In_ IDebugHostType* objectType,
        _Out_ Location* derivedLocation,
        _Out_ IDebugHostType** derivedType
        ) PURE;

    //*************************************************
    // IDebugHostSymbols2:
    //

    // DemangleSymbolName():
    //
    // Attempts to take the name of the given symbol, find an appropriate demangler for it based on what
    // the host knows about the symbols, and returns a demangled form of the symbol name.
    //
    // If an appropriate demangler cannot be found (because we do not recognized the symbol format/language/
    // compiler, E_NOINTERFACE may be returned.  Other errors may also be returned depending.
    //
    // Currently, flags are reserved and must be set to zero.
    //
    STDMETHOD(DemangleSymbolName)(
        THIS_
        _In_ IDebugHostSymbol *pSymbol,
        _In_ ULONG flags,
        _Out_ BSTR *pDemangledSymbolName
        ) PURE;
};

//
// IDebugHostMemory:
//
// The core memory access interface which a debug host presents.  This interface can be QI'd from
// IDebugHost in order to access memory regions (be they virtual / physical / registers / etc...)
//
// Note that the combination of context and "location" in the methods of this interface
// need not necessarily refer to the virtual address space of the target.  They can refer to the
// physical address space of the target, an I/O space of the target, a register space of the target, etc...
//
#undef INTERFACE
#define INTERFACE IDebugHostMemory
DECLARE_INTERFACE_(IDebugHostMemory, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostMemory:

    // ReadBytes():
    //
    // Reads a number of bytes from the address space of the target as defined by the inpassed context and location.
    // The number of bytes read is returned in "bytesRead" upon success.
    //
    STDMETHOD(ReadBytes)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _Out_writes_bytes_(bufferSize) void* buffer,
        _In_ ULONG64 bufferSize,
        _Out_opt_ ULONG64* bytesRead
        ) PURE;

    // WriteBytes():
    //
    // Writes a number of bytes to the address space of the target as defined by the inpassed context and location.
    // The number of bytes written is returned in "bytesWritten" upon success.
    //
    STDMETHOD(WriteBytes)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_reads_bytes_(bufferSize) void* buffer,
        _In_ ULONG64 bufferSize,
        _Out_opt_ ULONG64* bytesWritten
        ) PURE;

    // ReadPointers():
    //
    // Reads a number of pointer sized objects from the address space of the target as defined by the inpassed
    // context and location.
    //
    // Each read pointer is, if necessary, zero extended to 64-bits and returned.
    //
    STDMETHOD(ReadPointers)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_ ULONG64 count,
        _Out_writes_(count) ULONG64* pointers
        ) PURE;

    // WritePointers():
    //
    // Takes a number of pointers as held in unsigned 64-bit values, truncates them to the native pointer size
    // of the target, and writes them into the address space of the target as defined by the inpassed
    // context and location.
    //
    STDMETHOD(WritePointers)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_ ULONG64 count,
        _In_reads_(count) ULONG64* pointers
        ) PURE;

    // GetDisplayStringForLocation():
    //
    // For a given location within the address space of the target as defined by context and location,
    // convert the location to a displayable string (according to whatever format the host chooses).
    //
    // If the "verbose" argument is true, the string conversion may be "more verbose"
    //
    STDMETHOD(GetDisplayStringForLocation)(
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_ bool verbose,
        _Out_ BSTR* locationName
        ) PURE;
};

//
// IDebugHostMemory2:
//
// The second version of the core memory access interface which a debug host presents.
// This interface can be QI'd from IDebugHost in order to access memory regions (be they
// virtual / physical / registers / etc...)
//
// Note that the combination of context and "location" in the methods of this interface
// need not necessarily refer to the virtual address space of the target.  They can refer to the
// physical address space of the target, an I/O space of the target, a register space of the target, etc...
//
#undef INTERFACE
#define INTERFACE IDebugHostMemory2
DECLARE_INTERFACE_(IDebugHostMemory2, IDebugHostMemory)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostMemory:

    STDMETHOD(ReadBytes)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _Out_writes_bytes_(bufferSize) void* buffer,
        _In_ ULONG64 bufferSize,
        _Out_opt_ ULONG64* bytesRead
        ) PURE;

    STDMETHOD(WriteBytes)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_reads_bytes_(bufferSize) void* buffer,
        _In_ ULONG64 bufferSize,
        _Out_opt_ ULONG64* bytesWritten
        ) PURE;

    STDMETHOD(ReadPointers)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_ ULONG64 count,
        _Out_writes_(count) ULONG64* pointers
        ) PURE;

    STDMETHOD(WritePointers)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_ ULONG64 count,
        _In_reads_(count) ULONG64* pointers
        ) PURE;

    STDMETHOD(GetDisplayStringForLocation)(
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_ bool verbose,
        _Out_ BSTR* locationName
        ) PURE;

    //*************************************************
    // IDebugHostMemory2:

    // LinearizeLocation():
    //
    // Takes a location which may represent something other than a virtual memory address and attempts
    // to linearize the location into a virtual memory address within the given context.  This operation may fail
    // if the location cannot be represented by a virtual address (e.g.: it's a register).
    //
    STDMETHOD(LinearizeLocation)(
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _Out_ Location *pLinearizedLocation
        ) PURE;

};

#undef INTERFACE
#define INTERFACE IDebugHostMemory3
DECLARE_INTERFACE_(IDebugHostMemory3, IDebugHostMemory2)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostMemory:

    STDMETHOD(ReadBytes)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _Out_writes_bytes_(bufferSize) void* buffer,
        _In_ ULONG64 bufferSize,
        _Out_opt_ ULONG64* bytesRead
        ) PURE;

    STDMETHOD(WriteBytes)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_reads_bytes_(bufferSize) void* buffer,
        _In_ ULONG64 bufferSize,
        _Out_opt_ ULONG64* bytesWritten
        ) PURE;

    STDMETHOD(ReadPointers)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_ ULONG64 count,
        _Out_writes_(count) ULONG64* pointers
        ) PURE;

    STDMETHOD(WritePointers)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_ ULONG64 count,
        _In_reads_(count) ULONG64* pointers
        ) PURE;

    STDMETHOD(GetDisplayStringForLocation)(
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_ bool verbose,
        _Out_ BSTR* locationName
        ) PURE;

    //*************************************************
    // IDebugHostMemory2:

    STDMETHOD(LinearizeLocation)(
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _Out_ Location *pLinearizedLocation
        ) PURE;

    //*************************************************
    // IDebugHostMemory3:

    // CanonicalizeLocation():
    //
    // For a given type of location (virtual memory, register, etc...): if the location can be represented
    // in more than one form, this will pick one such form as the "canonical representation" and transform
    // the location to that canonical form.
    //
    // As an example: Debugging Tools for Windows (dbgeng) historically sign extends 32-bit addresses to 64-bits.
    // This results in difficulty representing and distinguishing certain address regions in high address aware
    // 32-bit processes.  As such, the canonical form of any address in the debug host layer is *ZERO EXTENDED*
    // to 64-bits.  
    //
    // Comparing locations or addresses gathered from components which report sign extended addresses against zero
    // extended ones will result in unexpected failures.  Calling to canonicalize beforehand will guarantee
    // no such error.
    //
    // This API will *NOT* transform the *type* of a location.  Calling LinearizeLocation will perform such and
    // guarantees the return of a canonical address.
    //
    STDMETHOD(CanonicalizeLocation)(
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _Out_ Location *pCanonicalizedLocation
        ) PURE;
};

#undef INTERFACE
#define INTERFACE IDebugHostMemory4
DECLARE_INTERFACE_(IDebugHostMemory4, IDebugHostMemory3)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostMemory:

    STDMETHOD(ReadBytes)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _Out_writes_bytes_(bufferSize) void* buffer,
        _In_ ULONG64 bufferSize,
        _Out_opt_ ULONG64* bytesRead
        ) PURE;

    STDMETHOD(WriteBytes)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_reads_bytes_(bufferSize) void* buffer,
        _In_ ULONG64 bufferSize,
        _Out_opt_ ULONG64* bytesWritten
        ) PURE;

    STDMETHOD(ReadPointers)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_ ULONG64 count,
        _Out_writes_(count) ULONG64* pointers
        ) PURE;

    STDMETHOD(WritePointers)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_ ULONG64 count,
        _In_reads_(count) ULONG64* pointers
        ) PURE;

    STDMETHOD(GetDisplayStringForLocation)(
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_ bool verbose,
        _Out_ BSTR* locationName
        ) PURE;

    //*************************************************
    // IDebugHostMemory2:

    STDMETHOD(LinearizeLocation)(
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _Out_ Location *pLinearizedLocation
        ) PURE;

    //*************************************************
    // IDebugHostMemory3:

    STDMETHOD(CanonicalizeLocation)(
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _Out_ Location *pCanonicalizedLocation
        ) PURE;

    //*************************************************
    // IDebugHostMemory4:
    //

    // GetPhysicalAddressLocation():
    //
    // Create a location structure representing an offset as a physical memory address.  
    // Note that this MAY legally fail (E_NOTIMPL) on a debugger which does not support physical addressing.  
    // If the debugger supports physical addressing but a specific target does not, 
    // GetPhysicalAddressLocation will succeed but attempts to read from that location will fail.
    //
    // If this method succeeds, the location can be utilized with any method that takes
    // a location (including the creation of typed objects at a physical address).
    //
    // Callers are free to manipulate the offset of the returned location on successful return
    // of the method.
    //
    STDMETHOD(GetPhysicalAddressLocation)(
        _In_ ULONG64 physAddr,
        _Out_ Location *pPhysicalAddressLocation
        ) PURE;

    // IsPhysicalAddressLocation():
    //
    // Returns whether a given location represents a physical address or not.
    //
    STDMETHOD_(bool, IsPhysicalAddressLocation)(
        _In_ Location *pLocation
        ) PURE;

};

#undef INTERFACE
#define INTERFACE IDebugHostMemory5
DECLARE_INTERFACE_(IDebugHostMemory5, IDebugHostMemory4)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostMemory:

    STDMETHOD(ReadBytes)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _Out_writes_bytes_(bufferSize) void* buffer,
        _In_ ULONG64 bufferSize,
        _Out_opt_ ULONG64* bytesRead
        ) PURE;

    STDMETHOD(WriteBytes)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_reads_bytes_(bufferSize) void* buffer,
        _In_ ULONG64 bufferSize,
        _Out_opt_ ULONG64* bytesWritten
        ) PURE;

    STDMETHOD(ReadPointers)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_ ULONG64 count,
        _Out_writes_(count) ULONG64* pointers
        ) PURE;

    STDMETHOD(WritePointers)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_ ULONG64 count,
        _In_reads_(count) ULONG64* pointers
        ) PURE;

    STDMETHOD(GetDisplayStringForLocation)(
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_ bool verbose,
        _Out_ BSTR* locationName
        ) PURE;

    //*************************************************
    // IDebugHostMemory2:

    STDMETHOD(LinearizeLocation)(
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _Out_ Location *pLinearizedLocation
        ) PURE;

    //*************************************************
    // IDebugHostMemory3:

    STDMETHOD(CanonicalizeLocation)(
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _Out_ Location *pCanonicalizedLocation
        ) PURE;

    //*************************************************
    // IDebugHostMemory4:
    //

    STDMETHOD(GetPhysicalAddressLocation)(
        _In_ ULONG64 physAddr,
        _Out_ Location *pPhysicalAddressLocation
        ) PURE;

    STDMETHOD_(bool, IsPhysicalAddressLocation)(
        _In_ Location *pLocation
        ) PURE;

    //*************************************************
    // IDebugHostMemory5:
    //

    // ReadIntrinsics():
    //
    // Reads one or more intrinsic values from the address space of the target as defined by the inpassed context
    // and location.  The number of intrinsics read is returned in the "intrinsicsRead" upon success.
    //
    // The type of intrinsics is given by the 'vt' value and can be one of:
    //
    //     VT_I1 - VT_I8   : signed integers
    //     VT_UI1 - VT_UI1 : unsigned integers
    //     VT_R4 - VT_R8   : standard floating point (single/double precision) values
    //
    STDMETHOD(ReadIntrinsics)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_ VARTYPE vt,
        _In_ ULONG64 count,
        _Out_writes_(count) VARIANT *vals,
        _Out_ ULONG64 *intrinsicsRead
        ) PURE;

    // ReadOrdinalIntrinsics():
    //
    // Similar to ReadIntrinsics(), the type of intrinsic is given by an ordinal size and indication of the
    // signed-ness of the intrinsics.  This only supports 8-64 bit signed and unsigned ordinals.
    //
    STDMETHOD(ReadOrdinalIntrinsics)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ Location location,
        _In_ ULONG64 ordinalSize,
        _In_ bool ordinalIsSigned,
        _In_ ULONG64 count,
        _Out_writes_(count) VARIANT *vals,
        _Out_ ULONG64 *intrinsicsRead
        ) PURE;
};


//
// IDebugHostEvaluator:
//
// The core expression evaluation interface which a debug host presents.  This interface can be QI'd from
// IDebugHost in order to perform expression evaluation.
//
// NOTE: In order to be as compatible as possible across the hosts which support this interface, callers
// of expression evaluation should either restrict their usage to pure language level semantics or should
// be prepared to fall back to other means.  Different hosts may extend expression valuation with non-language
// semantics as they see fit.  Such semantics are not guaranteed to be cross compatible.  Only pure language
// level semantics are.
//
#undef INTERFACE
#define INTERFACE IDebugHostEvaluator
DECLARE_INTERFACE_(IDebugHostEvaluator, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostEvaluator:

    // EvaluateExpression():
    //
    // Causes the host to evaluate an expression.  The input expression will be evaluated with
    // language syntax.  The underlying expression evaluator will not offer any extensions which
    // are private to a particular host.  Such expressions can be evaluated via an intentional
    // choice to call EvaluateExtendedExpression.
    //
    // The host context is passed in the "context" argument and
    // defines what debug target, etc...  the expression evaluation occurs in the context of.
    //
    // The "bindingContext" argument supplies an effective "this" pointer for the expression evaluation.
    // An evaluation of "m_foo" with a bindingContext of "bar" is effectively evaluating "bar.m_foo".
    // To remove a name in the expression from application of "this", it is necessary to qualify the name
    // via some form of qualification (e.g.: a global '::', module qualification syntax, etc...)
    //
    // If the "bindingContext" argument is nullptr, all names will be bound as if the host were in the state
    // described by the "context" argument.  This may include local variables of a specified stack frame
    // of a specified thread, globals (as determined by the host), etc...
    //
    // The result of the expression evaluation and, optionally any resultant metadata, are returned from
    // this method.  If an evaluation error occurs, it is possible for the method to return a failing HRESULT
    // and still return an error object in the "result" output argument.
    //
    STDMETHOD(EvaluateExpression)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ PCWSTR expression,
        _In_opt_ IModelObject* bindingContext,
        _COM_Errorptr_ IModelObject** result,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    // EvaluateExtendedExpression():
    //
    // Evaluates an expression in a similar fashion to EvaluateExpression().  Calling this method turns
    // on any features that the host's expression evaluator provides on top of language semantics.
    // These features are host specific and are not generic amongst all hosts which support a particular
    // language.
    //
    // Any extension which calls this method must handle failure of the method in a graceful manner.
    //
    STDMETHOD(EvaluateExtendedExpression)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ PCWSTR expression,
        _In_opt_ IModelObject* bindingContext,
        _COM_Errorptr_ IModelObject** result,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;
};

//
// SignatureComparison:
//
// Describes how a type or two signatures compare.
//
enum SignatureComparison
{
    // The two signatures/types being compared are unrelated.
    Unrelated,

    // One signature/type compares ambiguously against the other.  For instance,
    // std::pair<*, int> versus std::pair<int, *> are ambiguous.  There are types that would
    // match both of these equally well (e.g.: std::pair<int, int>)
    Ambiguous,

    // One signature/type is less specific than the other.  For instance, a comparison of
    // std::vector<*> against std::vector<int> would yield LessSpecific.
    LessSpecific,

    // One signature/type is more specific than the other.  For instance, a comparison of
    // std::vector<int> against std::vector<*> would yield MoreSpecific.
    MoreSpecific,

    // The signatures/types are identical.
    Identical
};

//
// IDebugHostModuleSignature:
//
// The interface which represents a module signature: A module name with an optional version range.
//
#undef INTERFACE
#define INTERFACE IDebugHostModuleSignature
DECLARE_INTERFACE_(IDebugHostModuleSignature, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        )PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        )PURE;

    //*************************************************
    // IDebugHostModuleSignature:

    STDMETHOD(IsMatch)(
        THIS_
        _In_ IDebugHostModule* pModule,
        _Out_ bool* isMatch
        ) PURE;
};

//
// IDebugHostTypeSignature:
//
// The interface which represents a type signature: a wildcarded generalization of a set of types.
// Models can be registered against type signatures so that they are automatically attached to any
// object instance whose type instance matches the signature.
//
#undef INTERFACE
#define INTERFACE IDebugHostTypeSignature
DECLARE_INTERFACE_(IDebugHostTypeSignature, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostTypeSignature:

    // GetHashCode():
    //
    // Gets a 32-bit hash code for this type signature.  Note that any type instance which matches
    // this particular signature (with the exception of global matches) *MUST* have the same hash code.
    //
    STDMETHOD(GetHashCode)(
        THIS_
        _Out_ ULONG* hashCode
        ) PURE;

    // IsMatch():
    //
    // Returns whether or not a particular type instance matches this signature.  If it does, true is returned
    // from "isMatch" and the optional "wildcardMatches" argument will be set to an enumerator which enumerates
    // all of the wildcard matches between the type instance and the type signature.
    //
    STDMETHOD(IsMatch)(
        THIS_
        _In_ IDebugHostType* type,
        _Out_ bool* isMatch,
        _COM_Outptr_opt_ IDebugHostSymbolEnumerator** wildcardMatches
        ) PURE;

    // CompareAgainst():
    //
    // Evaluates the relationship between two type signatures and returns information about that relationship.
    // One type signature can be unrelated, less specific, more specific, identical, or ambiguous as compared
    // to another.
    //
    // It is illegal to have ambiguity between two type signatures registered with the data model manager.
    //
    STDMETHOD(CompareAgainst)(
        THIS_
        _In_ IDebugHostTypeSignature* typeSignature,
        _Out_ SignatureComparison* result
        ) PURE;

};

// SymbolSearchOptions:
//
enum SymbolSearchOptions
{
    // SymbolSearchNone: No options set
    SymbolSearchNone = 0x00000000,

    // SymbolSearchCompletion: Search for symbols starting with the specified name rather than
    // symbols of the exact specified name.
    SymbolSearchCompletion = 0x00000001,

    // SymbolSearchCaseInsensitive: Search for symbols using case-insensitive rules.
    SymbolSearchCaseInsensitive = 0x00000002
};

// SymbolSearchInfo:
//
// The search record passed to EnumerateChildrenEx in order to restrict symbol searches.
// A given kind of symbol (as indicated by the SymbolKind enumeration) searched may have its own derived type.
//
struct SymbolSearchInfo
{
#ifdef __cplusplus
protected:
    SymbolSearchInfo(_In_ ULONG derivedSize) :
        HeaderSize(sizeof(SymbolSearchInfo)),
        InfoSize(derivedSize),
        SearchOptions(0)
    {
    }

public:
    SymbolSearchInfo() :
        HeaderSize(sizeof(SymbolSearchInfo)),
        InfoSize(sizeof(SymbolSearchInfo)),
        SearchOptions(0)
    {
    }
#endif // __cplusplus

    ULONG HeaderSize; // sizeof(SymbolSearchInfo)
    ULONG InfoSize;   // sizeof(* by SymbolKind *)

    ULONG SearchOptions;

    //
    // What follows is per SymbolKind:
    //
};

// TypeSearchInfo:
//
// The search record passed to EnumerateChildrenEx specifically for SymbolType searches.
//
struct TypeSearchInfo : public SymbolSearchInfo
{
#ifdef __cplusplus
    TypeSearchInfo() :
        SymbolSearchInfo(sizeof(TypeSearchInfo))
    {
    }

    TypeSearchInfo(_In_ TypeKind searchType) :
        SymbolSearchInfo(sizeof(TypeSearchInfo)),
        SearchType(searchType)
    {
    }
#endif // __cplusplus

    // Defines the type being searched for.
    TypeKind SearchType;
};

// LanguageKind:
//
// Identifies the language of the compiland containing a given symbol.
//
enum LanguageKind
{
    // LanguageUnknown: Indicates that the language cannot be identified
    LanguageUnknown,

    // LanguageC: Indicates the C language
    LanguageC,

    // LanguageCPP: Indicates the C++ language
    LanguageCPP,

    // LanguageAssembly: Indicates assembly
    LanguageAssembly,

    // LanguageRust: Indicates Rust
    LanguageRust
};

#undef INTERFACE
#define INTERFACE IDebugHostSymbol2
DECLARE_INTERFACE_(IDebugHostSymbol2, IDebugHostSymbol)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    STDMETHOD(CompareAgainst)(
        THIS_
        _In_ IDebugHostSymbol *pComparisonSymbol,
        _In_ ULONG comparisonFlags,
        _Out_ bool *pMatches
        ) PURE;

    //*************************************************
    // IDebugHostSymbol2

    // EnumerateChildrenEx():
    //
    // Enumerates all child symbols of the given type, name, and extended information which is present.
    // This behaves identically to EnumerateChildren when searchInfo is nullptr.  SymbolType::Symbol
    // can be used to search to search for any kind of child.
    //
    // Note that if name is nullptr, children of any name will be produced by the resulting enumerator.
    //
    STDMETHOD(EnumerateChildrenEx)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _In_opt_ SymbolSearchInfo* searchInfo,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    // GetLanguage():
    //
    // If the symbol can identify the language for which it applies, this returns an identifier for such.
    // Many symbols will *NOT* be able to make this determination.  In such cases, this method will fail.
    // It is also possible that the host does not understand the language or there is no defined LanguageKind.
    // In such cases, LanguageUnknown will be returned and the method will succeed.
    //
    STDMETHOD(GetLanguage)(
        THIS_
        _Out_ LanguageKind *pKind
        ) PURE;

};

#undef INTERFACE
#define INTERFACE IDebugHostSymbol3
DECLARE_INTERFACE_(IDebugHostSymbol3, IDebugHostSymbol2)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostSymbol:

    STDMETHOD(GetContext)(
        THIS_
        _COM_Outptr_ IDebugHostContext** context
        ) PURE;

    STDMETHOD(EnumerateChildren)(
        THIS_
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetSymbolKind)(
        THIS_
        _Out_ SymbolKind *kind
        ) PURE;

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* symbolName
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** type
        ) PURE;

    STDMETHOD(GetContainingModule)(
        THIS_
        _Out_ IDebugHostModule **containingModule
        ) PURE;

    STDMETHOD(CompareAgainst)(
        THIS_
        _In_ IDebugHostSymbol *pComparisonSymbol,
        _In_ ULONG comparisonFlags,
        _Out_ bool *pMatches
        ) PURE;

    //*************************************************
    // IDebugHostSymbol2

    STDMETHOD(EnumerateChildrenEx)(
        _In_ SymbolKind kind,
        _In_opt_z_ PCWSTR name,
        _In_opt_ SymbolSearchInfo* searchInfo,
        _Out_ IDebugHostSymbolEnumerator **ppEnum
        ) PURE;

    STDMETHOD(GetLanguage)(
        THIS_
        _Out_ LanguageKind *pKind
        ) PURE;

    //*************************************************
    // IDebugHostSymbol3:
    //

    // GetCompilerInformation():
    //
    // If the symbol can identify the compiler which produced it, this will return information
    // about that compiler.
    //
    // Note that the "compiler string" returned may be a compiler name or may include additional information
    // (e.g.: command line arguments, etc...).  That depends on the underlying implementation.
    // 
    // It is legal for a debug host to E_NOTIMPL this.
    //
    STDMETHOD(GetCompilerInformation)(
        THIS_
        _Out_ KnownCompiler *pCompilerId,
        _Out_opt_ BSTR *pCompilerString
        ) PURE;

};

//
// IDebugHostStatus
//
// This interface exposes the status of the underlying IDebugHost.
//
#undef INTERFACE
#define INTERFACE IDebugHostStatus
DECLARE_INTERFACE_(IDebugHostStatus, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostStatus:

    // PollUserInterrupt():
    //
    // Returns whether a user interrupt has been set. Ie: Ctrl+Break
    //
    STDMETHOD(PollUserInterrupt)(
        THIS_
        _Out_ bool* interruptRequested
        ) PURE;
};

//
// IDebugHostStatus2
//
// This interface exposes the status of the underlying IDebugHost.
//
#undef INTERFACE
#define INTERFACE IDebugHostStatus2
DECLARE_INTERFACE_(IDebugHostStatus2, IDebugHostStatus)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostStatus:

    STDMETHOD(PollUserInterrupt)(
        THIS_
        _Out_ bool* interruptRequested
        ) PURE;

    //*************************************************
    // IDebugHostStatus2:

    // SetUserInterrupt():
    //
    // Effect a "break" triggered by the user.
    //
    STDMETHOD(SetUserInterrupt)(
        THIS_
        ) PURE;

    // ClearUserInterrupt():
    //
    // Clear the flag indicating that a "break" triggered by the user has been handled.
    //
    STDMETHOD(ClearUserInterrupt)(
        THIS_
        ) PURE;
};

//**************************************************************************
// Script Provider and Scripting Interfaces:
//

// IDataModelScriptClient:
//
// A host or other client which wishes to interface with a scripting extension must have one
// or more implementations of IDataModelScriptClient in order to receive messages from the script
// provider.
//
#undef INTERFACE
#define INTERFACE IDataModelScriptClient
DECLARE_INTERFACE_(IDataModelScriptClient, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelScriptClient:

    // ReportError():
    //
    // Called from the provider when there is an error in the execution of the script.  The class of error,
    // error code, error message, and position of the error within the script are passed.  The client
    // decides what to do with the error information.
    //
    STDMETHOD(ReportError)(
        THIS_
        _In_ ErrorClass errClass,
        _In_ HRESULT hrFail,
        _In_opt_ PCWSTR message,
        _In_ ULONG line,
        _In_ ULONG position
        ) PURE;

};

// IDataModelScriptTemplate:
//
// If a script provider has template content that it wishes to advertise to the host, it implements
// this interface.
//
#undef INTERFACE
#define INTERFACE IDataModelScriptTemplate
DECLARE_INTERFACE_(IDataModelScriptTemplate, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelScriptTemplate:

    // GetName():
    //
    // Gets the name of the template content.  This may fail with E_NOTIMPL if the template does not
    // have a name.  A default template is not required to have a name.  All other templates must.
    //
    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR *templateName
        ) PURE;

    // GetDescription():
    //
    // Gets the description for this template content. This may fail with E_NOTIMPL if the template does
    // not have a description.
    //
    STDMETHOD(GetDescription)(
        THIS_
        _Out_ BSTR *templateDescription
        ) PURE;

    // GetContent():
    //
    // Returns a stream over the template content.
    //
    STDMETHOD(GetContent)(
        THIS_
        _COM_Outptr_ IStream **contentStream
        ) PURE;


};

// IDataModelScript:
//
// The interface which represents an individual script managed by a script provider.  Each script provider
// must be capable of creating and managing multiple scripts.
//
#undef INTERFACE
#define INTERFACE IDataModelScript
DECLARE_INTERFACE_(IDataModelScript, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelScript:

    // GetName():
    //
    // Gets the present "name" of the script.
    //
    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR *scriptName
        ) PURE;

    // Rename():
    //
    // Renames the script and, in so doing, may alter where the host projects the contents of the script
    // into the debugger namespace.
    //
    STDMETHOD(Rename)(
        THIS_
        _In_ PCWSTR scriptName
        ) PURE;

    // Populate():
    //
    // Called by the host (or client) in order to change/synchronize the "content" of the script.  This does *NOT*
    // cause execution of the script of any objects that the script manipulates.  It merely tells the script provider
    // that the content of the script has changed so that it may synchronize its own internal state.
    //
    // The implementer of the Populate method may not hold the content stream between the Populate and Execute calls.
    // It *MUST* synchronize any internal state and data structures during a Populate.
    //
    STDMETHOD(Populate)(
        THIS_
        _In_ IStream *contentStream
        ) PURE;

    // Execute():
    //
    // Executes the content of the script.  Note taht execution of the script is a two way communcation between the
    // script provider and the script client.  Errors, debugging control, and other semantics are passed across the
    // communication channel between IDataModelScript and IDataModelScriptClient.
    //
    // After a successful return, the script provider's projection of the script should be active.  Note that if the
    // script fails to execute (e.g.: syntax errors, etc...), the prior contents of the projection should *not*
    // disappear.
    //
    // For a properly written script provider and script environment, calling Execute multiple times without an
    // intervening Populate() or Unlink() call should be idempotent.  Execution of a script should create a bridge
    // between the object model of the debugger and that of the script.  It should not produce side effecting results
    // on the state of the debug target.   Utilizing properties, methods, or events on the bridge produced via
    // Execute may indeed produce side effecting results.
    //
    STDMETHOD(Execute)(
        THIS_
        _In_ IDataModelScriptClient *client
        ) PURE;

    // Unlink():
    //
    // Undoes the Execute operation.  Any links which were made between the script and the object model are unlinked.
    // After an Unlink operation, the script may be re-executed via a call to Execute or it may be released.
    //
    // It is expected that this is called, for instance, upon the closing of a script window by a UI client.
    //
    STDMETHOD(Unlink)(
        THIS
        ) PURE;

    // IsInvocable():
    //
    // Returns whether or not the script is invocable (e.g.: it has some semblance of a main function as defined by
    // its provider).  This method may only be called after a successful Execute.
    //
    // If this method returns true in its argument, InvokeMain() may be called.
    //
    STDMETHOD(IsInvocable)(
        THIS_
        _Out_ bool *isInvocable
        ) PURE;

    // InvokeMain():
    //
    // If the script has a main function which is intended to execute from a UI invocation, the provider can indicate
    // such in metadata and the call to this method will "invoke" the script.  Note that this is distinct from *Execute*
    // which runs all root code and bridges the script to the namespace of the underlying host.
    //
    // This method may fail with E_NOTIMPL if the script does not contain a "named main function" or the provider does
    // not define a "main" method.
    //
    STDMETHOD(InvokeMain)(
        THIS_
        _In_ IDataModelScriptClient *client
        ) PURE;

};

//
// IDataModelScript2:
//
// This is the second version of the core script interface.
//
#undef INTERFACE
#define INTERFACE IDataModelScript2
DECLARE_INTERFACE_(IDataModelScript2, IDataModelScript)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelScript:

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR *scriptName
        ) PURE;

    STDMETHOD(Rename)(
        THIS_
        _In_ PCWSTR scriptName
        ) PURE;

    STDMETHOD(Populate)(
        THIS_
        _In_ IStream *contentStream
        ) PURE;

    STDMETHOD(Execute)(
        THIS_
        _In_ IDataModelScriptClient *client
        ) PURE;

    STDMETHOD(Unlink)(
        THIS
        ) PURE;

    STDMETHOD(IsInvocable)(
        THIS_
        _Out_ bool *isInvocable
        ) PURE;

    STDMETHOD(InvokeMain)(
        THIS_
        _In_ IDataModelScriptClient *client
        ) PURE;

    //*************************************************
    // IDataModelScript2:

    // GetScriptFullFilePathName():
    //
    // Gets the script full file path name.
    //
    STDMETHOD(GetScriptFullFilePathName)(
        THIS_
        _Out_ BSTR *scriptFullPathName
        ) PURE;

    // SetScriptFullFilePathName():
    //
    // Sets the script full file path name. The method can be executed only once.
    //
    STDMETHOD(SetScriptFullFilePathName)(
        THIS_
        _In_ PCWSTR scriptFullPathName
        ) PURE;
};

//
// IDataModelScriptTemplateEnumerator:
//
// Core interface which enumerates any and all template content available from the provider for
// the creation of new prepopulated scripts.
//
#undef INTERFACE
#define INTERFACE IDataModelScriptTemplateEnumerator
DECLARE_INTERFACE_(IDataModelScriptTemplateEnumerator, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelScriptTemplateEnumerator:

    // Reset():
    //
    // Resets the enumerator to the first element.
    //
    STDMETHOD(Reset)(
        THIS
        ) PURE;

    // GetNext():
    //
    // Gets the next script template which is being enumerated and moves the enumerator
    // to the next position.  E_BOUNDS will be returned at the end of enumeration.
    //
    STDMETHOD(GetNext)(
        THIS_
        _COM_Outptr_ IDataModelScriptTemplate **templateContent
        ) PURE;
};

//
// IDataModelScriptProvider:
//
// The core interface that any extension which wishes to act as a script provider to the underlying host must
// implement.
//
#undef INTERFACE
#define INTERFACE IDataModelScriptProvider
DECLARE_INTERFACE_(IDataModelScriptProvider, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelScriptProvider:

    // GetName():
    //
    // Returns the name (type) of the script provider (e.g.: "JavaScript", "Python", "NatVis", etc...)
    //
    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR *name
        ) PURE;

    // GetExtension():
    //
    // Returns the extension which the script provider supports (e.g.: "js", "py", "NatVis", etc...).  Loads
    // of scripts with this extension will automatically be directed to the appropriately registered script
    // provider.
    //
    STDMETHOD(GetExtension)(
        THIS_
        _Out_ BSTR *extension
        ) PURE;

    // CreateScript():
    //
    // When the host wishes to create a script based on the provider, it calls this method.
    //
    STDMETHOD(CreateScript)(
        THIS_
        _COM_Outptr_ IDataModelScript **script
        ) PURE;

    // GetDefaultTemplateContent():
    //
    // Returns an object which represents the "default template" content for the scrtipt.  If the script
    // provider does not have any default template content, E_NOTIMPL may be returned from this interface.
    //
    STDMETHOD(GetDefaultTemplateContent)(
        THIS_
        _COM_Outptr_ IDataModelScriptTemplate **templateContent
        ) PURE;

    // EnumerateTemplates():
    //
    // Enumerates all varying templates that are available from the provider.  The default template
    // will enumerate in this set.  In addition, the provider may enumerate an arbitrary number of "named"
    // templates which have varying content prepopulated for differing purposes.
    //
    STDMETHOD(EnumerateTemplates)(
        THIS_
        _COM_Outptr_ IDataModelScriptTemplateEnumerator **enumerator
        ) PURE;

};

#undef INTERFACE
#define INTERFACE IDataModelScriptProviderEnumerator
DECLARE_INTERFACE_(IDataModelScriptProviderEnumerator, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelScriptProviderEnumerator:

    // Reset():
    //
    // Resets the enumerator to the first element.
    //
    STDMETHOD(Reset)(
        THIS
        ) PURE;

    // GetNext():
    //
    // Gets the next script provider which is being enumerated and moves the enumerator
    // to the next position.  E_BOUNDS will be returned at the end of enumeration.
    //
    STDMETHOD(GetNext)(
        THIS_
        _COM_Outptr_ IDataModelScriptProvider **provider
        ) PURE;
};

#undef INTERFACE
#define INTERFACE IDataModelScriptManager
DECLARE_INTERFACE_(IDataModelScriptManager, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelScriptManager:

    // GetDefaultNameBinder():
    //
    // Gets the default name binder for script providers.  This returns an object which
    // can bind a name in the context of an object and does so according to a default set
    // of rules.  Script providers can make use of this provider to provide consistency
    // in name resolution across providers.
    //
    STDMETHOD(GetDefaultNameBinder)(
        THIS_
        _COM_Outptr_ IDataModelNameBinder **ppNameBinder
        ) PURE;

    // RegisterScriptProvider():
    //
    // An extension which wishes to register itself as a provider of scripting functionality to
    // the host calls this method in order to register itself.  Once registered, the provider
    // should work in all contexts that general scripting works in the underlying debugger host.
    //
    // Such registration may fail if there is already a provider registered under a particular name
    // or for a particular script extension.
    //
    STDMETHOD(RegisterScriptProvider)(
        THIS_
        _In_ IDataModelScriptProvider *provider
        ) PURE;

    // UnregisterScriptProvider():
    //
    // Called by an extension which wishes to remove its registration of being a script provider.  Note
    // that if scripts are still loaded that reference the provider, they will continue to function.
    //
    // This only prevents the creation or loading of new scripts.
    //
    STDMETHOD(UnregisterScriptProvider)(
        THIS_
        _In_ IDataModelScriptProvider *provider
        ) PURE;

    // FindProviderForScriptType():
    //
    // Finds a provider by its registered name (e.g.: "JavaScript", "Python", "NatVis", etc...).
    // If no such provider can be found, E_BOUNDS is returned.
    //
    // Note that the comparison on 'scriptType' is case insensitive.
    //
    STDMETHOD(FindProviderForScriptType)(
        THIS_
        _In_ PCWSTR scriptType,
        _COM_Outptr_ IDataModelScriptProvider **provider
        ) PURE;

    // FindProviderForScriptExtension():
    //
    // Finds a provider by its script extension registration (e.g.: "js", "py", "NatVis", etc...).
    // If no such provider can be found, E_BOUNDS is returned.
    //
    // Note that the comparison on 'scriptExtension' is case insensitive.
    //
    STDMETHOD(FindProviderForScriptExtension)(
        THIS_
        _In_ PCWSTR scriptExtension,
        _COM_Outptr_ IDataModelScriptProvider **provider
        ) PURE;

    // EnumerateScriptProviders():
    //
    // Returns an enumerator for all registered script providers in the system.  A client
    // which wishes to have UI to allow for the creation of arbitrary language scripts can
    // utilize this to determine which providers are present in the system.
    //
    STDMETHOD(EnumerateScriptProviders)(
        THIS_
        _COM_Outptr_ IDataModelScriptProviderEnumerator **enumerator
        ) PURE;

};

//
// IDynamicKeyProviderConcept
//
// Any object which provides a dynamic set of keys and values (e.g.: a bridge to a dynamic scripting language)
// can be a key provider.
//
// Unlike most concepts, a client should not GetConcept(IID_IDynamicKeyProviderConcept).  Doing so returns the **FIRST**
// dynamic key provider on the object.  The data model will internally use this concept on each model as it enumerates
// keys.
//
#undef INTERFACE
#define INTERFACE IDynamicKeyProviderConcept
DECLARE_INTERFACE_(IDynamicKeyProviderConcept, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDynamicKeyProviderConcept:

    // GetKey():
    //
    // Returns the key from the dynamic key provider and any metadata associated with that
    // key.  In the event that key is not present (but no other error occurs), the provider
    // must return *hasKey = false / S_OK.  A failure of this call is considered a failure
    // to fetch a key and will explicitly halt the search for the key.  Returning *hasKey = false / S_OK
    // will not stop the search.
    //
    // It is perfectly legal for GetKey to return a boxed property accessor as the key.  This would
    // be semantically identical to an IModelObject::GetKey returning a property accessor.
    //
    STDMETHOD(GetKey)(
        THIS_
        _In_ IModelObject *contextObject,
        _In_ PCWSTR key,
        _COM_Outptr_opt_result_maybenull_ IModelObject** keyValue,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata,
        _Out_opt_ bool *hasKey
        ) PURE;

    // SetKey():
    //
    // Sets a key in the dynamic provider.  This is considered the creation of a new property on the
    // provider.  Note that a provider which does not support any notion of something like the creation
    // of expand properties should return E_NOTIMPL here.
    //
    STDMETHOD(SetKey)(
        THIS_
        _In_ IModelObject *contextObject,
        _In_ PCWSTR key,
        _In_ IModelObject *keyValue,
        _In_ IKeyStore *metadata
        ) PURE;

    // EnumerateKeys():
    //
    // Enumerates the keys within the dynamic key provider.  The returned enumerator must behave as per an
    // EnumerateKeys(...) call on IModelObject and not as EnumerateKeyValues or any of the other enumeration
    // variants.
    //
    // Note that from the perspective of a single dynamic key provider, it is illegal to enumerate multiple keys
    // of the same name that are physically distinct keys.
    //
    STDMETHOD(EnumerateKeys)(
        THIS_
        _In_ IModelObject *contextObject,
        _COM_Outptr_ IKeyEnumerator **ppEnumerator
        ) PURE;

};

//
// IDynamicConceptProviderConcept
//
// Any object which provides a dynamic set of concepts and implementations (e.g.: a bridge to a dynamic scripting language)
// can be a concept provider.  There are consequences and limitations of being a dynamic concept provider:
//
//     - An object can only be declared to be a dynamic concept provider **BEFORE** any other non IDynamic* concepts are
//       added to the object.
//
//     - Once you declare that an object is a dynamic concept provider, that fact can *NEVER* be changed!
//
//     - An object which is a dynamic concept provider only has a single parent model.  That parent model is presented
//       to the dynamic concept provider to utilize.  All calls to query/change parents affect this single parent model
//       and *NOT* the object itself.
//
//     - The provider must decide how to deal with parent models that are added dynamically based on concepts.
//
// Unlike most concepts, a client should not GetConcept(IID_IDynamicConceptProviderConcept).  Doing so returns the **FIRST**
// dynamic concept provider on the object.  The data model will internally use this concept on each model as it looks up
// concepts.
//
// A dynamic concept provider can **NEVER** return any IDynamic*Concept interface!  You cannot have a dynamic concept provider
// say that it is a dynamic key provider, for instance.
//
#undef INTERFACE
#define INTERFACE IDynamicConceptProviderConcept
DECLARE_INTERFACE_(IDynamicConceptProviderConcept, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDynamicConceptProviderConcept:

    // GetConcept():
    //
    // Returns the concept from the dynamic concept provider and any metadata associated with that
    // concept.  In the event that the concept is not present (but no other error occurs), the provider
    // must return *hasConcept = false / S_OK.  A failure of this call is considered a failure
    // to fetch a concept and will explicitly halt the search for the concept.  Returning *hasConcept = false / S_OK
    // will not stop the search.
    //
    STDMETHOD(GetConcept)(
        THIS_
        _In_ IModelObject *contextObject,
        _In_ REFIID conceptId,
        _COM_Outptr_result_maybenull_ IUnknown **conceptInterface,
        _COM_Outptr_opt_result_maybenull_ IKeyStore **conceptMetadata,
        _Out_ bool *hasConcept
        ) PURE;

    // SetConcept():
    //
    // Sets a concept in the dynamic provider.  Note that a provider which does not allow the creation
    // of the concept shoiuld return E_NOTIMPL here.
    //
    STDMETHOD(SetConcept)(
        THIS_
        _In_ IModelObject *contextObject,
        _In_ REFIID conceptId,
        _In_ IUnknown *conceptInterface,
        _In_opt_ IKeyStore *conceptMetadata
        ) PURE;

    // NotifyParent():
    //
    // Notifies the concept provider of the **SINGLE** parent model which is present on the provider.
    //
    STDMETHOD(NotifyParent)(
        THIS_
        _In_ IModelObject *parentModel
        ) PURE;

    // NotifyParentChange():
    //
    // Notifies the concept provider that a static change has been made to the set of parents.
    //
    STDMETHOD(NotifyParentChange)(
        THIS_
        _In_ IModelObject *parentModel
        ) PURE;

    // NotifyDestruct():
    //
    // Notifies that the object which houses the dynamic concept provider is destructing.  This allows
    // a dynamic concept provider to clear caches if need be.
    //
    STDMETHOD(NotifyDestruct)(
        THIS
        ) PURE;

};

// ScriptChangeKind:
//
// Indicates the type of notification firing to the host.
//
enum ScriptChangeKind
{
    // The script has been renamed
    ScriptRename
};

// IDataModelScriptHostContext:
//
// A host context for a given script.
//
#undef INTERFACE
#define INTERFACE IDataModelScriptHostContext
DECLARE_INTERFACE_(IDataModelScriptHostContext, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelScriptHostContext:

    // NotifyScriptChange():
    //
    // Notifies the host of a change to the script.
    //
    STDMETHOD(NotifyScriptChange)(
        THIS_
        _In_ IDataModelScript* script,
        _In_ ScriptChangeKind changeKind
        ) PURE;

    // GetNamespaceObject():
    //
    // Gets the namespace object that the host has provided for the script.
    //
    STDMETHOD(GetNamespaceObject)(
        THIS_
        _COM_Outptr_ IModelObject** namespaceObject
        ) PURE;

};

//
// IDebugHostScriptHost:
//
// Any host which supports the data model's notion of scripting must provide an implementation of this interface.
// Script providers utilize this to bridge to the namespace of the host.
//
#undef INTERFACE
#define INTERFACE IDebugHostScripthost
DECLARE_INTERFACE_(IDebugHostScriptHost, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostScriptHost:

    // CreateContext():
    //
    // Called in order to create a context for a script.  A context from the host includes
    // a namespace object where the contents of the script can be bridged.
    //
    STDMETHOD(CreateContext)(
        THIS_
        _In_ IDataModelScript* script,
        _COM_Outptr_ IDataModelScriptHostContext** scriptContext
        ) PURE;

};

//
// IDataModelNameBinder:
//
// The default name binder for script providers.
//
#undef INTERFACE
#define INTERFACE IDataModelNameBinder
DECLARE_INTERFACE_(IDataModelNameBinder, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelNameBinder:

    // BindValue():
    //
    // Performs the equivalent of "contextObject . 'name'" on the given object according
    // to a default set of binding rules.  Such a default binder provides consistency between
    // script providers.
    //
    // The binding result is the result by value.
    //
    STDMETHOD(BindValue)(
        THIS_
        _In_ IModelObject* contextObject,
        _In_ PCWSTR name,
        _COM_Errorptr_ IModelObject** value,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    // BindReference():
    //
    // Performs the equivalent of "contextObject . 'name'" on the given object according
    // to a default set of binding rules.  Such a default binder provides consistency between
    // script providers.
    //
    // The binding result is the result by reference.
    //
    STDMETHOD(BindReference)(
        THIS_
        _In_ IModelObject* contextObject,
        _In_ PCWSTR name,
        _COM_Errorptr_ IModelObject** reference,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    // EnumerateValues():
    //
    // Enumerates the set of names and values which will bind against the object according
    // to BindValue(...).
    //
    // Unlike the standard "EnumerateKeys" and "EnumerateRawValues" and the like which may
    // return multiple names with the same value (for base classes, parent models, and the like),
    // this enumerator will *ONLY* return the specific set of names which will bind with
    // BindValue and BindReference.  Names will never be duplicated.
    //
    // There is a significantly higher cost in providing this functionality.
    //
    STDMETHOD(EnumerateValues)(
        THIS_
        _In_ IModelObject* contextObject,
        _COM_Outptr_ IKeyEnumerator** enumerator
        ) PURE;

    // EnumerateReferences():
    //
    // Enumerates the set of names and values which will bind against the object according
    // to BindReference(...).
    //
    // Unlike the standard "EnumerateKeys" and "EnumerateRawValues" and the like which may
    // return multiple names with the same value (for base classes, parent models, and the like),
    // this enumerator will *ONLY* return the specific set of names which will bind with
    // BindValue and BindReference.  Names will never be duplicated.
    //
    // There is a significantly higher cost in providing this functionality.
    //
    STDMETHOD(EnumerateReferences)(
        THIS_
        _In_ IModelObject* contextObject,
        _COM_Outptr_ IKeyEnumerator** enumerator
        ) PURE;

};

//
// IModelKeyReference2:
//
// Second version of IModelKeyReference
// Represents a reference to a key which can be resolved (dereferenced) to get the underlying key.
//
// This interface is never directly implemented by a client.
//
#undef INTERFACE
#define INTERFACE IModelKeyReference2
DECLARE_INTERFACE_(IModelKeyReference2, IModelKeyReference)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IModelKeyReference:

    STDMETHOD(GetKeyName)(
        THIS_
        _Out_ BSTR* keyName
        ) PURE;

    STDMETHOD(GetOriginalObject)(
        THIS_
        _COM_Outptr_ IModelObject** originalObject
        ) PURE;

    STDMETHOD(GetContextObject)(
        THIS_
        _COM_Outptr_ IModelObject** containingObject
        ) PURE;

    //*************************************************
    // Accessors:
    //

    STDMETHOD(GetKey)(
        THIS_
        _COM_Errorptr_opt_ IModelObject** object,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    STDMETHOD(GetKeyValue)(
        THIS_
        _COM_Errorptr_opt_ IModelObject** object,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    STDMETHOD(SetKey)(
        THIS_
        _In_opt_ IModelObject* object,
        _In_opt_ IKeyStore* metadata
        ) PURE;

    STDMETHOD(SetKeyValue)(
        THIS_
        _In_ IModelObject* object
        ) PURE;

    //*************************************************
    // IModelKeyReference2:

    // OverrideContextObject:
    //
    // Changes the context object which is used to resolve the key reference.  If the key reference
    // refers to a key which is a property (or to a dynamic key provider), 'newContextObject' as indicated
    // by a call to this method will be passed to the property getter or the dynamic key provider in
    // lieu of the context object from the key fetch.
    //
    // This is useful to certain script providers when building cross-language inheritence chains.
    //
    STDMETHOD(OverrideContextObject)(
        THIS_
        _In_ IModelObject* newContextObject
        ) PURE;
};

#undef INTERFACE
#define INTERFACE IDebugHostEvaluator2
DECLARE_INTERFACE_(IDebugHostEvaluator2, IDebugHostEvaluator)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostEvaluator:

    STDMETHOD(EvaluateExpression)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ PCWSTR expression,
        _In_opt_ IModelObject* bindingContext,
        _COM_Errorptr_ IModelObject** result,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    STDMETHOD(EvaluateExtendedExpression)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ PCWSTR expression,
        _In_opt_ IModelObject* bindingContext,
        _COM_Errorptr_ IModelObject** result,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    //*************************************************
    // IDebugHostEvaluator2:

    // AssignTo():
    //
    // For a caller which has a model based reference to a *language* value, evaluate
    // (assignmentReference = assignmentValue) and return the result of the assignment
    // according to the underlying language semantics.
    //
    STDMETHOD(AssignTo)(
        THIS_
        _In_ IModelObject* assignmentReference,
        _In_ IModelObject* assignmentValue,
        _COM_Errorptr_ IModelObject** assignmentResult,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** assignmentMetadata
        ) PURE;
};

#undef INTERFACE
#define INTERFACE IDebugHostEvaluator3
DECLARE_INTERFACE_(IDebugHostEvaluator3, IDebugHostEvaluator2)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostEvaluator:

    STDMETHOD(EvaluateExpression)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ PCWSTR expression,
        _In_opt_ IModelObject* bindingContext,
        _COM_Errorptr_ IModelObject** result,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    STDMETHOD(EvaluateExtendedExpression)(
        THIS_
        _In_ IDebugHostContext* context,
        _In_ PCWSTR expression,
        _In_opt_ IModelObject* bindingContext,
        _COM_Errorptr_ IModelObject** result,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** metadata
        ) PURE;

    //*************************************************
    // IDebugHostEvaluator2:

    STDMETHOD(AssignTo)(
        THIS_
        _In_ IModelObject* assignmentReference,
        _In_ IModelObject* assignmentValue,
        _COM_Errorptr_ IModelObject** assignmentResult,
        _COM_Outptr_opt_result_maybenull_ IKeyStore** assignmentMetadata
        ) PURE;

    //*************************************************
    // IDebugHostEvaluator3:

    // Compare():
    //
    // For a caller which wants to compare two model based objects for equality
    // linguistically, check if the two objects are equal. Handles pointers and
    // pointer coercion equality if necessary.
    STDMETHOD(Compare)(
        THIS_
        _In_ IModelObject *pLeft,
        _In_ IModelObject *pRight,
        _COM_Outptr_ IModelObject **ppResult
        ) PURE;
};

// IDebugHostExtensibility:
//
// An interface which allows the callers to extend capabilities of the host based on the data model.
// This interface and all its methods are optional for the host to implement.
//
#undef INTERFACE
#define INTERFACE IDebugHostExtensibility
DECLARE_INTERFACE_(IDebugHostExtensibility, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostExtensibility:

    // CreateFunctionAlias():
    //
    // For a function that the script wants to make a "quick alias" for, allow the host to create
    // such an alias.  The meaning of such an alias is host specific.  It may, for instance, mean
    // that the alias name is available as a root function in the host's expression evaluator.  It
    // may mean that calling the function can be done through "!alias" at a command line.
    //
    STDMETHOD(CreateFunctionAlias)(
        THIS_
        _In_ PCWSTR aliasName,
        _In_ IModelObject *functionObject
        ) PURE;

    // DestroyFunctionAlias():
    //
    // Destroys a previously created function alias.
    //
    STDMETHOD(DestroyFunctionAlias)(
        THIS_
        _In_ PCWSTR aliasName
        ) PURE;

};

// IDebugHostExtensibility2
//
// An interface which allows the callers to extend capabilities of the host based on the data model.
// This interface and all its methods are optional for the host to implement.
//
#undef INTERFACE
#define INTERFACE IDebugHostExtensibility2
DECLARE_INTERFACE_(IDebugHostExtensibility2, IDebugHostExtensibility)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostExtensibility:

    // CreateFunctionAlias():
    //
    STDMETHOD(CreateFunctionAlias)(
        THIS_
        _In_ PCWSTR aliasName,
        _In_ IModelObject *functionObject
        ) PURE;

    // DestroyFunctionAlias():
    //
    STDMETHOD(DestroyFunctionAlias)(
        THIS_
        _In_ PCWSTR aliasName
        ) PURE;


    //*************************************************
    // IDebugHostExtensibility2:

    // CreateFunctionAliasWithMetadata():
    //
    // Same as CreateFunctionAlias but allows passing metadata information about the function.
    //
    STDMETHOD(CreateFunctionAliasWithMetadata)(
        THIS_
        _In_ PCWSTR aliasName,
        _In_ IModelObject *functionObject,
        _In_opt_ IKeyStore* metadata
        ) PURE;
};

#undef INTERFACE
#define INTERFACE IDebugHostExtensibility3
DECLARE_INTERFACE_(IDebugHostExtensibility3, IDebugHostExtensibility2)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostExtensibility:

    // CreateFunctionAlias():
    //
    STDMETHOD(CreateFunctionAlias)(
        THIS_
        _In_ PCWSTR aliasName,
        _In_ IModelObject *functionObject
        ) PURE;

    // DestroyFunctionAlias():
    //
    STDMETHOD(DestroyFunctionAlias)(
        THIS_
        _In_ PCWSTR aliasName
        ) PURE;


    //*************************************************
    // IDebugHostExtensibility2:

    // CreateFunctionAliasWithMetadata():
    //
    STDMETHOD(CreateFunctionAliasWithMetadata)(
        THIS_
        _In_ PCWSTR aliasName,
        _In_ IModelObject *functionObject,
        _In_opt_ IKeyStore* metadata
        ) PURE;

    //*************************************************
    // IDebugHostExtensibility3:

    // ExtendHostContext():
    //
    // For hosts which support placing arbitrary POD data inside a host context, this method extends the
    // notion of a host context with a specific size of POD data.  The data stored inside a host context must
    // be plain intrinsic data: it cannot be a COM interface; it cannot be anything which requires destruction;
    // it should be as small as possible.
    //
    // This method will return a unique identifier which can be used to set and retrieve such data on any
    // host context object.  Unless it is explicitly added via IDebugHostContextExtensibility, no host context 
    // will contain such data.
    //
    STDMETHOD(ExtendHostContext)(
        THIS_
        _In_ ULONG blobSize,
        _In_ REFGUID identifier,
        _Out_ ULONG *blobId
        ) PURE;

    // QueryHostContextExtension():
    //
    // This looks up a context extension and its information by the identifier GUID.
    //
    STDMETHOD(QueryHostContextExtension)(
        THIS_
        _In_ REFGUID identifier,
        _Out_ ULONG *blobId,
        _Out_ ULONG *blobSize
        ) PURE;

    // ReleaseHostContextExtension():
    //
    // Releases the reservation of a host context blob as acquired from ExtendHostContext.
    //
    STDMETHOD(ReleaseHostContextExtension)(
        THIS_
        _In_ ULONG blobId
        ) PURE;
};

// enum ScriptDebugState:
//
// Defines the current debugging state of a script
//
enum ScriptDebugState
{
    // ScriptDebugNoDebugger: indicates that debugging is not active on the script
    ScriptDebugNoDebugger,

    // ScriptDebugNotExecuting: indicates that no code within the script is actively executing
    ScriptDebugNotExecuting,

    // ScriptDebugExecuting: indicates that the script is executing code
    ScriptDebugExecuting,

    // ScriptDebugBreak: the script status is that it is broken into the script debugger
    ScriptDebugBreak
};

// enum ScriptDebugEventFilter:
//
// Defines the set of debug events / exceptions which the script debugger can (potentially) auto-break
// on
//
enum ScriptDebugEventFilter
{
	// ScriptDebugEventFilterEntry: Indicates that a break on *EVERY ENTRY* into the script from outside should break into the debugger
    ScriptDebugEventFilterEntry,

    // ScriptDebugEventFilterException: Indicates that any first chance exception should immediately break into the debugger
    ScriptDebugEventFilterException,

    // ScriptDebugEventFilterUnhandledException: Indicates that unhandled exceptions should immediately break into the debugger
    ScriptDebugEventFilterUnhandledException,

    // ScriptDebugEventFilterAbort: Indicates that an abort (core debugger BREAK/STOP this action) should break into the script
    //                              debugger rather than aborting the script execution
    ScriptDebugEventFilterAbort
};

// enum ScriptDebugEvent:
//
// Defines what debug event occurred.
//
enum ScriptDebugEvent
{
    // ScriptDebugBreakpoint: indicates that a breakpoint was hit
    ScriptDebugBreakpoint,

    // ScriptDebugStep: indicates that a step event occurred
    ScriptDebugStep,

    // ScriptDebugException: indicates that an exception occurred.  The ScriptDebugEventInformation will fill in .u.ExceptionInformation
    //                       and the outpassed object is a data model conversion of the actual exception
    ScriptDebugException,

    // ScriptDebugAsyncBreak: indicates that a break into the script occurred (either because of something like break on entry, break on abort,
    //                        etc...)
    ScriptDebugAsyncBreak

};

// enum ScriptExecutionKind
//
// Defines the kind of execution to do.
//
enum ScriptExecutionKind
{
    // ScriptExecutionNormal: indicates that you would like the script to execute normally
    ScriptExecutionNormal,

    // ScriptExecutionStepIn: indicates that you want to perform a "step in" operation
    ScriptExecutionStepIn,

    // ScriptExecutionStepOut: indicates that you want to perform a "step out" operation
    ScriptExecutionStepOut,

    // ScriptExecutionStepOver: indicates that you want to perform a "step over" operation
    ScriptExecutionStepOver,

};

// ScriptDebugPosition:
//
// Defines a position within a script.
//
struct ScriptDebugPosition
{
    ULONG Line;                 // A zero value indicates that the line cannot be determined
    ULONG Column;               // A zero value indicates that the column cannot be determined
};

// ScriptDebugEventInformation:
//
// A simple struct containing information about a particular debug event.
//
struct ScriptDebugEventInformation
{
    ScriptDebugEvent DebugEvent;
    ScriptDebugPosition EventPosition;          // The line/column of script at which the debug event occurred (0 values : cannot determine)
    ScriptDebugPosition EventSpanEnd;           // The ending line/column of script at which the debug event occurred (0 values : cannot determine)

    union
    {
        struct
        {
            bool IsUncaught;
        } ExceptionInformation;

        struct
        {
            ULONG64 BreakpointId;
        } BreakpointInformation;

    } u;
};

// IDataModelScriptDebugClient:
//
// Any client utilizing the debugger for a particular script must implement this interface.  The interface
// will be called back to process any debug events.  Such callback will happen synchronously during the execution
// of any script content.
//
#undef INTERFACE
#define INTERFACE IDataModelScriptDebugClient
DECLARE_INTERFACE_(IDataModelScriptDebugClient, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelScriptDebugClient

    //
    // The debug client gets notified about any debug event which occurs.  The resume kind is normally
    // a ScriptExecutionNormal but it can be changed by returning a different resume kind.
    //
    STDMETHOD(NotifyDebugEvent)(
        _In_ ScriptDebugEventInformation *pEventInfo,
        _In_ IDataModelScript *pScript,
        _In_opt_ IModelObject *pEventDataObject,
        _Inout_ ScriptExecutionKind *resumeEventKind
        ) PURE;

};

// IDataModelScriptDebugVariableSetEnumerator:
//
// Enumerates a set of variables (arguments, parameters, locals, etc...)
//
#undef INTERFACE
#define INTERFACE IDataModelScriptDebugVariableSetEnumerator
DECLARE_INTERFACE_(IDataModelScriptDebugVariableSetEnumerator, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelScriptDebugVariableSetEnumerator:

    // Reset():
    //
    // Resets the enumerator to the first variable in the set.
    //
    STDMETHOD(Reset)(
        THIS
        ) PURE;

    // GetNext():
    //
    // Gets the next element in the set.  In certain cases, the variable "name" may be null
    // on output (e.g.: for arguments which are not named)
    //
    STDMETHOD(GetNext)(
        THIS_
        _Out_ BSTR *variableName,
        _COM_Outptr_opt_ IModelObject **variableValue,
        _COM_Outptr_opt_result_maybenull_ IKeyStore **variableMetadata
        ) PURE;

};

// IDataModelScriptDebugStackFrame:
//
// Represents a stack frame from the script provider.
//
#undef INTERFACE
#define INTERFACE IDataModelScriptDebugStackFrame
DECLARE_INTERFACE_(IDataModelScriptDebugStackFrame, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelScriptDebugStackFrame:

    // GetName():
    //
    // Gets the display name (e.g.: function name) of the frame.
    //
    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR *name
        ) PURE;

    // GetPosition():
    //
    // Gets the position of the execution within this frame in the script.  This may **ONLY** be called when the script is within a break
    // represented by this stack.
    //
    // The position of the execution within this frame is always returned.  If the particular debugger is capable of returning the span
    // of the "execution position" within the script, an ending position can be returned in positionSpanEnd.  If the debugger
    // is not capable of this, the line and column values in the span end (if requested) should be set to zero.
    //
    // The line text can optionally be passed back by debug engines which support it.  There is no requirement that a script
    // debugger be able to return more than line and column position.  If this is not supported, nullptr can be returned.
    //
    STDMETHOD(GetPosition)(
        THIS_
        _Out_ ScriptDebugPosition *position,
        _Out_opt_ ScriptDebugPosition *positionSpanEnd,
        _Out_opt_ BSTR *lineText
        ) PURE;

    // IsTransitionPoint():
    //
    // Returns whether this stack frame is a transition point into another script.  It is perfectly legal to E_NOTIMPL this
    // particular method.
    //
    STDMETHOD(IsTransitionPoint)(
        THIS_
        _Out_ bool *isTransitionPoint
        ) PURE;

    // GetTransition():
    //
    // Returns the script that this frame was transitioned into from.  Note that a particular script debugger may not be able to
    // detect discontinuity outside its own context.
    //
    // If the script debugger can detect whether the call came from the script passed, isTransitionContiguous should be set to true.
    // If it is not (or cannot detect), isTransitionContiguous should be set to false.
    //
    // It is perfectly legal to E_NOTIMPL this particular method.
    //
    STDMETHOD(GetTransition)(
        THIS_
        _COM_Outptr_ IDataModelScript **transitionScript,
        _Out_ bool *isTransitionContiguous
        ) PURE;

    // Evaluate():
    //
    // Evaluates an expression in the context of this stack frame and returns the result as a model object.
    //
    STDMETHOD(Evaluate)(
        THIS_
        _In_ PCWSTR pwszExpression,
        _COM_Outptr_ IModelObject **ppResult
        ) PURE;

    // EnumerateLocals():
    //
    // Enumerates locals in the frame.
    //
    STDMETHOD(EnumerateLocals)(
        THIS_
        _COM_Outptr_ IDataModelScriptDebugVariableSetEnumerator **variablesEnum
        ) PURE;

    // EnumerateArguments():
    //
    // Enumerates arguments to the function in this frame.
    //
    STDMETHOD(EnumerateArguments)(
        THIS_
        _COM_Outptr_ IDataModelScriptDebugVariableSetEnumerator **variablesEnum
        ) PURE;
};

// IDataModelScriptDebugStack:
//
// Represents a stack from the script provider.
//
#undef INTERFACE
#define INTERFACE IDataModelScriptDebugStack
DECLARE_INTERFACE_(IDataModelScriptDebugStack, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelScriptDebugStack:

    // GetFrameCount():
    //
    // Gets the number of stack frames within this stack.
    //
    STDMETHOD_(ULONG64, GetFrameCount)(
        THIS
        ) PURE;

    // GetStackFrame():
    //
    // Gets a stack frame within the stack.
    //
    STDMETHOD(GetStackFrame)(
        THIS_
        _In_ ULONG64 frameNumber,
        _COM_Outptr_ IDataModelScriptDebugStackFrame **stackFrame
        ) PURE;
};

// IDataModelScriptDebugBreakpoint:
//
// Represents a breakpoint within the script.
//
#undef INTERFACE
#define INTERFACE IDataModelScriptDebugBreakpoint
DECLARE_INTERFACE_(IDataModelScriptDebugBreakpoint, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelScriptDebugBreakpoint:

    // GetId():
    //
    // Gets a unique identifier for this breakpoint **WITHIN THE CONTEXT** of this script.  The breakpoint ID may
    // be unique beyond that depending on the provider but the only requirement is that it be unique within
    // its script.
    //
    STDMETHOD_(ULONG64, GetId)(
        THIS
        ) PURE;

    // IsEnabled():
    //
    // Indicates whether or not the breakpoint is enabled.
    //
    STDMETHOD_(bool, IsEnabled)(
        THIS
        ) PURE;

    // Enable():
    //
    // Enables the breakpoint.
    //
    STDMETHOD_(void, Enable)(
        THIS
        ) PURE;

    // Disable():
    //
    // Disables the breakpoint.
    //
    STDMETHOD_(void, Disable)(
        THIS
        ) PURE;

    // Remove():
    //
    // Removes the breakpoint from its containing list.
    //
    STDMETHOD_(void, Remove)(
        THIS
        ) PURE;

    // GetPosition():
    //
    // Gets the position of the breakpoint within the script.
    //
    STDMETHOD(GetPosition)(
        _Out_ ScriptDebugPosition *position,
        _Out_opt_ ScriptDebugPosition *positionSpanEnd,
        _Out_opt_ BSTR *lineText
        ) PURE;

};

// IDataModelScriptDebugBreakpointEnumerator:
//
// An enumeration of a set of breakpoints.
//
#undef INTERFACE
#define INTERFACE IDataModelScriptDebugBreakpointEnumerator
DECLARE_INTERFACE_(IDataModelScriptDebugBreakpointEnumerator, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelScriptDebugBreakpointEnumerator:

    // Reset():
    //
    // Resets the enumerator
    //
    STDMETHOD(Reset)(
        THIS
        ) PURE;

    // GetNext():
    //
    // Gets the next breakpoint.
    //
    STDMETHOD(GetNext)(
        THIS_
        _COM_Outptr_ IDataModelScriptDebugBreakpoint **breakpoint
        ) PURE;

};

// IDataModelScriptDebug:
//
// An optional interface on scripts (objects which implement IDataModelScript).  If this interface is supported,
// the script provider has a debug engine capable of some limited form of 'script debugging'.  This may include
// stepping, setting breakpoints, inspecting data, etc...
//
#undef INTERFACE
#define INTERFACE IDataModelScriptDebug
DECLARE_INTERFACE_(IDataModelScriptDebug, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelScriptDebug:

    // GetDebugState():
    //
    //
    STDMETHOD_(ScriptDebugState, GetDebugState)(
        THIS
        ) PURE;

    // GetCurrentPosition():
    //
    // Gets the current position of the script.  This may **ONLY** be called when the script is broken into the debugger.
    // A call otherwise will fail.
    //
    // The current position of the script is always returned.  If the particular debugger is capable of returning the span
    // of the "current position" within the script, an ending position can be returned in positionSpanEnd.  If the debugger
    // is not capable of this, the line and column values in the span end (if requested) should be set to zero.
    //
    // The line text can optionally be passed back by debug engines which support it.  There is no requirement that a script
    // debugger be able to return more than line and column position.  If this is not supported, nullptr can be returned.
    //
    STDMETHOD(GetCurrentPosition)(
        THIS_
        _Out_ ScriptDebugPosition *currentPosition,
        _Out_opt_ ScriptDebugPosition *positionSpanEnd,
        _Out_opt_ BSTR *lineText
        ) PURE;

    // GetStack():
    //
    // Gets the current stack at the break position.  This may **ONLY** be called when the script is broken into the debugger.
    //
    STDMETHOD(GetStack)(
        THIS_
        _COM_Outptr_ IDataModelScriptDebugStack **stack
        ) PURE;

    // SetBreakpoint():
    //
    // Sets a breakpoint within the script.
    //
    STDMETHOD(SetBreakpoint)(
        THIS_
        _In_ ULONG linePosition,
        _In_ ULONG columnPosition,
        _COM_Outptr_ IDataModelScriptDebugBreakpoint **breakpoint
        ) PURE;

    // FindBreakpointById():
    //
    // Finds a breakpoint by its identifier.
    //
    STDMETHOD(FindBreakpointById)(
        THIS_
        _In_ ULONG64 breakpointId,
        _COM_Outptr_ IDataModelScriptDebugBreakpoint **breakpoint
        ) PURE;

    // EnumerateBreakpoints():
    //
    // Enumerates all breakpoints on the script in question.
    //
    STDMETHOD(EnumerateBreakpoints)(
        THIS_
        _COM_Outptr_ IDataModelScriptDebugBreakpointEnumerator **breakpointEnum
        ) PURE;

    // GetEventFilter():
    //
    // Gets the event filter (break on event) status for a particular event.
    //
    STDMETHOD(GetEventFilter)(
        THIS_
        _In_ ScriptDebugEventFilter eventFilter,
        _Out_ bool *isBreakEnabled
        ) PURE;

    // SetEventFilter():
    //
    // Sets the current event filter (break on event) status for a particular event.  Some script engines may return
    // E_NOTIMPL for particular events to indicate that the particular debug engine does not support the particular
    // event.
    //
    STDMETHOD(SetEventFilter)(
        THIS_
        _In_ ScriptDebugEventFilter eventFilter,
        _In_ bool isBreakEnabled
        ) PURE;

    // StartDebugging():
    //
    // Called by a client which wishes to start debugging a script.  The act of starting debugging does not actively
    // cause any execution break or stepping.  It merely makes the script debuggable and provides a set of interfaces
    // for the client to communicate with the script debugger.
    //
    STDMETHOD(StartDebugging)(
        THIS_
        _In_ IDataModelScriptDebugClient *debugClient
        ) PURE;

    // StopDebugging():
    //
    // Called by a client which wishes to stop debugging a script.  This may be called from a break status or
    // a running status of the script.  It immediately ceases all debugging activity and resets the state back to before
    // StartDebugging was called.
    //
    STDMETHOD(StopDebugging)(
        THIS_
        _In_ IDataModelScriptDebugClient *debugClient
        ) PURE;

};

// IDataModelScriptDebug2:
//
// An optional interface on scripts (objects which implement IDataModelScript).  If this interface is supported,
// the script provider has a debug engine capable of some limited form of 'script debugging'.  This may include
// stepping, setting breakpoints, inspecting data, etc...
//
#undef INTERFACE
#define INTERFACE IDataModelScriptDebug2
DECLARE_INTERFACE_(IDataModelScriptDebug2, IDataModelScriptDebug)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDataModelScriptDebug:

    STDMETHOD_(ScriptDebugState, GetDebugState)(
        THIS
        ) PURE;

    STDMETHOD(GetCurrentPosition)(
        THIS_
        _Out_ ScriptDebugPosition *currentPosition,
        _Out_opt_ ScriptDebugPosition *positionSpanEnd,
        _Out_opt_ BSTR *lineText
        ) PURE;

    STDMETHOD(GetStack)(
        THIS_
        _COM_Outptr_ IDataModelScriptDebugStack **stack
        ) PURE;

    STDMETHOD(SetBreakpoint)(
        THIS_
        _In_ ULONG linePosition,
        _In_ ULONG columnPosition,
        _COM_Outptr_ IDataModelScriptDebugBreakpoint **breakpoint
        ) PURE;

    STDMETHOD(FindBreakpointById)(
        THIS_
        _In_ ULONG64 breakpointId,
        _COM_Outptr_ IDataModelScriptDebugBreakpoint **breakpoint
        ) PURE;

    STDMETHOD(EnumerateBreakpoints)(
        THIS_
        _COM_Outptr_ IDataModelScriptDebugBreakpointEnumerator **breakpointEnum
        ) PURE;

    STDMETHOD(GetEventFilter)(
        THIS_
        _In_ ScriptDebugEventFilter eventFilter,
        _Out_ bool *isBreakEnabled
        ) PURE;

    STDMETHOD(SetEventFilter)(
        THIS_
        _In_ ScriptDebugEventFilter eventFilter,
        _In_ bool isBreakEnabled
        ) PURE;

    STDMETHOD(StartDebugging)(
        THIS_
        _In_ IDataModelScriptDebugClient *debugClient
        ) PURE;

    STDMETHOD(StopDebugging)(
        THIS_
        _In_ IDataModelScriptDebugClient *debugClient
        ) PURE;

    //*************************************************
    // IDataModelScriptDebug2

    // SetBreakpointAtFunction():
    //
    // Sets a breakpoint on the function given by the supplied name.
    //
    STDMETHOD(SetBreakpointAtFunction)(
        THIS_
        _In_ PCWSTR functionName,
        _COM_Outptr_ IDataModelScriptDebugBreakpoint **breakpoint
        ) PURE;
};

#undef INTERFACE
#define INTERFACE IComparableConcept
DECLARE_INTERFACE_(IComparableConcept, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IComparableConcept:

    // CompareObjects():
    //
    // Compares this object to another (of arbitrary type).  If the comparison
    // cannot be performed, E_NOT_SET should be returned.
    //
    // The return value passed in comparison result has the following meaning:
    //
    //    < 0 : contextObject < otherObject
    //      0 : contextObject == otherObject
    //    > 0 : contextObject > otherObject
    //
    STDMETHOD(CompareObjects)(
        THIS_
        _In_ IModelObject *contextObject,
        _In_ IModelObject *otherObject,
        _Out_ int *comparisonResult
        ) PURE;
};

#undef INTERFACE
#define INTERFACE IEquatableConcept
DECLARE_INTERFACE_(IEquatableConcept, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IEquatableConcept:

    // AreObjectsEqual():
    //
    // Compares this object to another (of arbitrary type) for equality.  If
    // the comparison cannot be performed, E_NOT_SET should be returned.
    //
    STDMETHOD(AreObjectsEqual)(
        THIS_
        _In_ IModelObject *contextObject,
        _In_ IModelObject *otherObject,
        _Out_ bool *isEqual
        ) PURE;
};

// IActionEnumerator:
//
// An enumerator for actions on an object.
//
#undef INTERFACE
#define INTERFACE IActionEnumerator
DECLARE_INTERFACE_(IActionEnumerator, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IActionEnumerator:

    // Reset():
    //
    // Resets the enumerator to the first action.
    //
    STDMETHOD(Reset)(
        THIS
        ) PURE;

    // GetNext():
    //
    // Gets the next action on the object.
    //
    STDMETHOD(GetNext)(
        THIS_
        _Out_ BSTR *keyName,
        _Out_ BSTR *actionName,
        _Out_ BSTR *actionDescription,
        _Out_ bool *actionIsDefault,
        _COM_Outptr_opt_ IModelObject **actionMethod,
        _COM_Outptr_opt_ IKeyStore **metadta
        ) PURE;
};

// IActionableConcept:
//
// A concept mechanism for implementing actions.  Clients may choose to either implement this interface or place
// appropriate metadata on effective void(void) methods.
//
// While this concept may be implemented, clients wishing to enumerate actions should not directly query for this
// concept.  Rather, they should query for IActionQueryConcept
//
#undef INTERFACE
#define INTERFACE IActionableConcept
DECLARE_INTERFACE_(IActionableConcept, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IActionableConcept:

    // EnumerateActions():
    //
    // Returns an enumerator to all actions on this object.
    //
    STDMETHOD(EnumerateActions)(
        THIS_
        _In_ IModelObject *contextObject,
        _COM_Outptr_ IActionEnumerator **actionEnumerator
        ) PURE;
};

// IActionQueryConcept:
//
// A concept which is automatically implemented by the data model for any object which has (or can have) actions
// on it.  The enumerator returned from this concept will aggregate all actions implemented via metadata keys on
// methods and those implemented via direct support of IActionableConcept anywhere on the object.
//
// Clients should *NEVER* implement this concept -- only query for it.
//
#undef INTERFACE
#define INTERFACE IActionQueryConcept
DECLARE_INTERFACE_(IActionQueryConcept, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IActionQueryConcept:

    // EnumerateActions():
    //
    // Returns an enumerator to all actions on this object.
    //
    STDMETHOD(EnumerateActions)(
        THIS_
        _In_ IModelObject *contextObject,
        _COM_Outptr_ IActionEnumerator **actionEnumerator
        ) PURE;
};

// IConstructableConcept:
//
// A concept that a data model can support in order to allow for construction of the object.
// Such a data model *MUST* support IDataModelConcept and *MUST* be registered under a name
// that is returned from IDataModelConcept::GetName.
//
#undef INTERFACE
#define INTERFACE IConstructableConcept
DECLARE_INTERFACE_(IConstructableConcept, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IConstructableConcept:
    //

    // CreateInstance():
    //
    // Calls to create an instance of this object/model.
    //
    STDMETHOD(CreateInstance)(
        THIS_
        _In_ ULONG64 argCount,
        _In_reads_(argCount) IModelObject **ppArguments,
        _COM_Errorptr_ IModelObject **ppInstance
        ) PURE;
};

// IDeconstructableConcept:
//
// A concept that a data model can support in order to decompose an object into a set
// of arguments which can be passed to the constructable concept in order to create a new
// identical instance of the object (short any extensions which were manually attached)
//
// An object which is indexable via a custom "value type" can support the deconstructable
// concept on object in order to allow a debugger engine to "serialize" enough information to
// get the indexer back in a subsequent invocation.
//
// Any object which supports the deconstructable concept should have a parent model attached
// which supports the constructable concept.  That model should be registered under the name
// returned from the GetConstructableModelName method on this interface.  The inverse is not necessarily true.
//
#undef INTERFACE
#define INTERFACE IDeconstructableConcept
DECLARE_INTERFACE_(IDeconstructableConcept, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDeconstructableConcept:
    //

    // GetConstructableModelName():
    //
    // Returns the name of a registered data model which supports the constructable concept.  Invocation
    // of the CreateInstance method on that constructable concept passing the arguments returned from
    // GetConstructorArguments here should "recreate" the an equivalent object (short any extensions
    // which were made after the fact).
    //
    STDMETHOD(GetConstructableModelName)(
        THIS_
        _In_ IModelObject* contextObject,
        _Out_ BSTR* constructableModelName
        ) PURE;

    // GetConstructorArgumentCount():
    //
    // Returns the number of constructor arguments that the object decomposes to.
    //
    STDMETHOD(GetConstructorArgumentCount)(
        THIS_
        _In_ IModelObject* contextObject,
        _Out_ ULONG64* argCount
        ) PURE;

    // GetConstructorArguments():
    //
    // Returns a set of arguments which, if passed back to the constructable concept, will
    // create an equivalent instance of the object (short any extensions which have been
    // attached after the fact).
    //
    STDMETHOD(GetConstructorArguments)(
        THIS_
        _In_ IModelObject* contextObject,
        _In_ ULONG64 argCount,
        _Out_writes_(argCount) IModelObject** constructorArguments
        ) PURE;
};


//**************************************************************************
// Optional Introspection Interfaces:
//

// StorageKind:
//
// Defines where a local is stored.
//
enum StorageKind
{
    StorageUnknown,
    StorageRegister,
    StorageRegisterRelative,
    StorageRegisterRelativeIndirect
};

#undef INTERFACE
#define INTERFACE IDebugHostFunctionLocalStorage
DECLARE_INTERFACE_(IDebugHostFunctionLocalStorage, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostFunctionLocalStorage:

    // GetValidRange():
    //
    // Gets a set of (module) relative addresses for which this storage is valid and
    // whether it is guaranteed within said range.
    //
    STDMETHOD(GetValidRange)(
        THIS_
        _Out_ ULONG64 *start,
        _Out_ ULONG64 *end,
        _Out_ bool *guaranteed
        ) PURE;

    // GetStorageKind():
    //
    // Gets the storage kind of the local.
    //
    STDMETHOD(GetStorageKind)(
        THIS_
        _Out_ StorageKind *kind
        ) PURE;

    // GetRegister():
    //
    // Gets the register that the local is stored within (or what register it is relative to).
    // The value returned here is architecture specific.
    //
    STDMETHOD(GetRegister)(
        THIS_
        _Out_ ULONG *registerId
        ) PURE;

    // GetOffset():
    //
    // Gets the offset from the register that the local is stored.
    //
    STDMETHOD(GetOffset)(
        THIS_
        _Out_ LONG64 *offset
        ) PURE;

};

#undef INTERFACE
#define INTERFACE IDebugHostFunctionLocalStorage2
DECLARE_INTERFACE_(IDebugHostFunctionLocalStorage2, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostFunctionLocalStorage2:


    // GetExtendedRegisterAddressInfo():
    //
    // Gets the register address information that the local is stored within (or what register it is relative to).
    // The values returned here are architecture specific.
    //
    STDMETHOD(GetExtendedRegisterAddressInfo)(
        THIS_
        _Out_ ULONG *registerId,
        _Out_ LONG64 *offset,
        _Out_ bool *isIndirectAccess,
        _Out_ LONG *indirectOffset
        ) PURE;
};

//
// IDebugHostFunctionLocalStorageEnumerator
//
#undef INTERFACE
#define INTERFACE IDebugHostFunctionLocalStorageEnumerator
DECLARE_INTERFACE_(IDebugHostFunctionLocalStorageEnumerator, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostFunctionLocalStorageEnumerator:

    // Reset():
    //
    // Resets the enumerator.
    //
    STDMETHOD(Reset)(
        THIS
        ) PURE;

    // GetNext():
    //
    // Gets information about the next local variable within the function.  Note
    // that this is "scopeless".  Two locals may be enumerated with the same name
    // because they are in different scopes.
    //
    STDMETHOD(GetNext)(
        THIS_
        _Out_ IDebugHostFunctionLocalStorage** storage
        ) PURE;
};

// LocalKind:
//
// Defines the kind of local that a particular name is (whether an argument to the function or a local
// variable)
//
enum LocalKind
{
    LocalArgument,
    LocalVariable
};

//
// IDebugHostFunctionLocalDetails:
//
// A host optional interface which provides details about a function local
// variable.
//
#undef INTERFACE
#define INTERFACE IDebugHostFunctionLocalDetails
DECLARE_INTERFACE_(IDebugHostFunctionLocalDetails, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostFunctionLocalDetails:

    // GetName():
    //
    // Gets the name of the local
    //
    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* name
        ) PURE;

    // GetType():
    //
    // Gets the type of the local
    //
    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** localType
        ) PURE;

    // EnumerateStorage():
    //
    // Enumerates the storage for the local (what registers or memory locations
    // it may be in within the function)
    //
    STDMETHOD(EnumerateStorage)(
        THIS_
        _Out_ IDebugHostFunctionLocalStorageEnumerator** storageEnum
        ) PURE;

    // GetLocalKind():
    //
    // Gets the kind of local that has been enumerated.
    //
    STDMETHOD(GetLocalKind)(
        THIS_
        _Out_ LocalKind* kind
        ) PURE;

    // GetArgumentPosition():
    //
    // Gets the position of a function argument within the argument list.  This method will fail on any
    // local which does not return LocalArgument from GetLocalKind().
    //
    STDMETHOD(GetArgumentPosition)(
        THIS_
        _Out_ ULONG64* argPosition
        ) PURE;
};

#undef INTERFACE
#define INTERFACE IDebugHostFunctionLocalDetails2
DECLARE_INTERFACE_(IDebugHostFunctionLocalDetails2, IDebugHostFunctionLocalDetails)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostFunctionLocalDetails:

    STDMETHOD(GetName)(
        THIS_
        _Out_ BSTR* name
        ) PURE;

    STDMETHOD(GetType)(
        THIS_
        _Out_ IDebugHostType** localType
        ) PURE;

    STDMETHOD(EnumerateStorage)(
        THIS_
        _Out_ IDebugHostFunctionLocalStorageEnumerator** storageEnum
        ) PURE;

    STDMETHOD(GetLocalKind)(
        THIS_
        _Out_ LocalKind* kind
        ) PURE;

    STDMETHOD(GetArgumentPosition)(
        THIS_
        _Out_ ULONG64* argPosition
        ) PURE;

    //*************************************************
    // IDebugHostFunctionLocalDetails2:

    // IsInlineScope():
    //
    // Is this local within an inlined scope.  This will always return false unless 
    // IDebugHostFunctionLocalDetailsEnumerator::EnumerateLocalsDetailsEx is called with
    // 'enumerateInlinedLocals' set to true.
    //
    STDMETHOD_(bool, IsInlineScope)(
        THIS
        ) PURE;

    // GetInlinedFunction():
    //
    // If IsInlineScope() returns true, this will return a symbol for the inlined function that the
    // local is contained within.
    //
    // This would be one of the symbols returned from a call to EnumerateInlineFunctionsByRVA within
    // IDebugHostFunctionLocalDetailsEnumerator.
    //
    // Note that if the local is not within an inline scope, this will return E_FAIL.
    //
    STDMETHOD(GetInlinedFunction)(
        THIS_
        _COM_Outptr_ IDebugHostSymbol** inlineFunction
        ) PURE;
};

//
// IDebugHostFunctionLocalDetailsEnumerator:
//
// A host optional interface which enumerates locals & arguments of a function and
// provides details about their backing storage and types.
//
#undef INTERFACE
#define INTERFACE IDebugHostFunctionLocalDetailsEnumerator
DECLARE_INTERFACE_(IDebugHostFunctionLocalDetailsEnumerator, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostFunctionLocalsDetailEnumerator:

    // Reset():
    //
    // Resets the enumerator.
    //
    STDMETHOD(Reset)(
        THIS
        ) PURE;

    // GetNext():
    //
    // Gets information about the next local variable within the function.  Note
    // that this is "scopeless".  Two locals may be enumerated with the same name
    // because they are in different scopes.
    //
    STDMETHOD(GetNext)(
        THIS_
        _Out_ IDebugHostFunctionLocalDetails** localDetails
        ) PURE;
};

//
// IDebugHostFunctionIntrospection:
//
// A host optional interface which provides detailed information about a function.
//
#undef INTERFACE
#define INTERFACE IDebugHostFunctionIntrospection
DECLARE_INTERFACE_(IDebugHostFunctionIntrospection, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostFunctionIntrospection:

    // EnumerateLocalsDetails():
    //
    // Enumerates a set of information about all locals and arguments within a function.  This will *NOT*
    // enumerate any locals and arguments of functions inlined within the given function.  EnumerateLocalsDetailsEx
    // on IDebugHostFunctionIntrospection2 can be used for such.
    //
    STDMETHOD(EnumerateLocalsDetails)(
        THIS_
        _Out_ IDebugHostFunctionLocalDetailsEnumerator** localsEnum
        ) PURE;

    // EnumerateInlineFunctionsByRVA():
    //
    // Enumerates the functions inlined at a particular RVA.  The functions will be
    // enumerated from the innermost containing inline function to the outermost. Thus,
    // the following:
    //
    // void Function1() {...};
    // void Function2()
    // {
    //     ...
    //     Function1();  //  inlined
    //     ...
    // }
    // void Function3()
    // {
    //     ...
    //     Function2();  //  inlined
    //     ...
    // }
    //
    // will result in this method emuerating Function1, followed by Function2, when called
    // on an inlined RVA for Function1, and will enumerate only Function2 when called on an
    // RVA inlined for Function2.
    //
    STDMETHOD(EnumerateInlineFunctionsByRVA)(
        THIS_
        _In_ ULONG64 rva,
        _Out_ IDebugHostSymbolEnumerator** inlinesEnum
        ) PURE;

    // FindCodeRangeForRVA():
    //
    // Returns the sub-range of instructions that contains a given address.  For single-
    // block functions, this will simply be the start and end of the function body. For
    // multi-block functions, this will be the start and end of the block containing the
    // specified RVA.
    //
    STDMETHOD(FindContainingCodeRangeByRVA)(
        THIS_
        _In_ ULONG64 rva,
        _Out_ Location* rangeStart,
        _Out_ Location* rangeEnd
        ) PURE;

    // FindSourceInformationByRVA():
    //
    // Returns the source file name and line number for a particular RVA.
    //
    STDMETHOD(FindSourceLocationByRVA)(
        THIS_
        _In_ ULONG64 rva,
        _Out_ BSTR* sourceFile,
        _Out_ ULONG64* sourceLine
        ) PURE;
};

//
// IDebugHostFunctionIntrospection2:
//
// A host optional interface which provides detailed information about a function.
//
#undef INTERFACE
#define INTERFACE IDebugHostFunctionIntrospection2
DECLARE_INTERFACE_(IDebugHostFunctionIntrospection2, IDebugHostFunctionIntrospection)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostFunctionIntrospection:

    STDMETHOD(EnumerateLocalsDetails)(
        THIS_
        _Out_ IDebugHostFunctionLocalDetailsEnumerator** localsEnum
        ) PURE;

    STDMETHOD(EnumerateInlineFunctionsByRVA)(
        THIS_
        _In_ ULONG64 rva,
        _Out_ IDebugHostSymbolEnumerator** inlinesEnum
        ) PURE;

    STDMETHOD(FindContainingCodeRangeByRVA)(
        THIS_
        _In_ ULONG64 rva,
        _Out_ Location* rangeStart,
        _Out_ Location* rangeEnd
        ) PURE;

    STDMETHOD(FindSourceLocationByRVA)(
        THIS_
        _In_ ULONG64 rva,
        _Out_ BSTR* sourceFile,
        _Out_ ULONG64* sourceLine
        ) PURE;

    //*************************************************
    // IDebugHostFunctionIntrospection2:
    //

    // EnumerateLocalsDetailsEx():
    //
    // Enumerates a set of information about all locals and arguments within a function.  This can also optionally
    // enumerate all locals and arguments of functions inlined within the given function if 'enumerateInlinedLocals'
    // is true.  If such argument is false, this method behaves as EnumerateLocalsDetails.
    //
    STDMETHOD(EnumerateLocalsDetailsEx)(
        THIS_
        _In_ bool enumerateInlinedLocals,
        _Out_ IDebugHostFunctionLocalDetailsEnumerator** localsEnum
        ) PURE;
};

//
// IDebugHostFunctionIntrospection3:
//
// A host optional interface which provides detailed information about a function.
//
#undef INTERFACE
#define INTERFACE IDebugHostFunctionIntrospection3
DECLARE_INTERFACE_(IDebugHostFunctionIntrospection3, IDebugHostFunctionIntrospection2)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IDebugHostFunctionIntrospection:

    STDMETHOD(EnumerateLocalsDetails)(
        THIS_
        _Out_ IDebugHostFunctionLocalDetailsEnumerator** localsEnum
        ) PURE;

    STDMETHOD(EnumerateInlineFunctionsByRVA)(
        THIS_
        _In_ ULONG64 rva,
        _Out_ IDebugHostSymbolEnumerator** inlinesEnum
        ) PURE;

    STDMETHOD(FindContainingCodeRangeByRVA)(
        THIS_
        _In_ ULONG64 rva,
        _Out_ Location* rangeStart,
        _Out_ Location* rangeEnd
        ) PURE;

    STDMETHOD(FindSourceLocationByRVA)(
        THIS_
        _In_ ULONG64 rva,
        _Out_ BSTR* sourceFile,
        _Out_ ULONG64* sourceLine
        ) PURE;

    //*************************************************
    // IDebugHostFunctionIntrospection2:
    //

    STDMETHOD(EnumerateLocalsDetailsEx)(
        THIS_
        _In_ bool enumerateInlinedLocals,
        _Out_ IDebugHostFunctionLocalDetailsEnumerator** localsEnum
        ) PURE;

    //*************************************************
    // IDebugHostFunctionIntrospection3:
    //

    // IsNoReturnFunction():
    //
    // Indicates whether or not the function is a non-returning function.
    //
    STDMETHOD(IsNoReturnFunction)(
        THIS_
        _Out_ bool *pIsNoReturnFunction
        ) PURE;

};

//
// IFilteredNamespacePropertyToken:
//
// Provides extension to data model functionality for manipulating namespaces
//

#undef INTERFACE
#define INTERFACE IFilteredNamespacePropertyToken
DECLARE_INTERFACE_(IFilteredNamespacePropertyToken, IUnknown)
{
    //*************************************************
    // IUnknown:

    STDMETHOD(QueryInterface)(
        THIS_
        _In_ REFIID iid,
        _COM_Outptr_ PVOID* iface
        ) PURE;

    STDMETHOD_(ULONG, AddRef)(
        THIS
        ) PURE;

    STDMETHOD_(ULONG, Release)(
        THIS
        ) PURE;

    //*************************************************
    // IFilteredNamespacePropertyToken:

    // RemoveFilter():
    //
    // Removes (irreversable) the filter from the namespace property
    //
    STDMETHOD(RemoveFilter)(
        THIS_
        ) PURE;

    // GetFilter():
    //
    // Gets the filter (if any)
    //
    STDMETHOD(GetFilter)(
        THIS_
        _COM_Outptr_result_maybenull_ IModelMethod **ppFilter
        ) PURE;

    // TrySetFilter():
    //
    // Sets a new filter if the previous filter is nullptr
    //
    STDMETHOD(TrySetFilter)(
        THIS_
        _In_ IModelMethod *pFilter
        ) PURE;
};


//**************************************************************************
// Bridge Interfaces to Target Composition Interfaces:
//
// These are optional.  A debug host which only supports the data model layer need not support any of these
// interfaces.  A debug host which supports both the data model layer and target composition layer of extensibility
// supports these to bridge between the two layers.
//

// {3D06878F-97AB-4c5b-955E-FA647D3B137C}
DEFINE_GUID(IID_IDebugHostContextTargetComposition, 0x3d06878f, 0x97ab, 0x4c5b, 0x95, 0x5e, 0xfa, 0x64, 0x7d, 0x3b, 0x13, 0x7c);

// {3C4B6ADD-80E1-4c2b-AFE1-9A1132586DD0}
DEFINE_GUID(IID_IDebugHostSymbolsTargetComposition, 0x3c4b6add, 0x80e1, 0x4c2b, 0xaf, 0xe1, 0x9a, 0x11, 0x32, 0x58, 0x6d, 0xd0);

struct DECLSPEC_UUID("3C4B6ADD-80E1-4c2b-AFE1-9A1132586DD0") IDebugHostSymbolsTargetComposition;
struct DECLSPEC_UUID("3D06878F-97AB-4c5b-955E-FA647D3B137C") IDebugHostContextTargetComposition;

//
// IDebugHostContextTargetComposition
//
// An interface which bridges the extensibility of the upper edge interfaces (the data model) with those of the
// lower edge (target composition) for a particular host context.
//
// For a debug host which supports both layers, this interface can be QI'd off any host context.
//
#undef INTERFACE
#define INTERFACE IDebugHostContextTargetComposition
DECLARE_INTERFACE_(IDebugHostContextTargetComposition, IUnknown)
{
    //*************************************************
    // IDebugHostContextTargetComposition
    //

    // GetServiceManager():
    //
    // For this particular host context, get the service manager container that is associated with the context.
    // If the context refers to something "debugger centric" above the level of a specific debug target, this
    // method will return an E_FAIL.
    //
    IFACEMETHOD(GetServiceManager)(
        THIS_
        _COM_Outptr_ IDebugServiceManager **ppServiceManager
        ) PURE;

    // GetServiceProcess():
    //
    // For this particular host context, get the process that is associated with the context.  If the context 
    // refers to something above the level of a specific process, this method will return an E_FAIL.
    //
    IFACEMETHOD(GetServiceProcess)(
        THIS_ 
        _COM_Outptr_ ISvcProcess **ppProcess
        ) PURE;

    // GetServiceThread():
    //
    // For this particular host context, get the thread that is associated with the context.  If the context
    // refers to something above the level of a specific thread (most contexts do), this method will return
    // an E_FAIL.
    //
    IFACEMETHOD(GetServiceThread)(
        THIS_
        _COM_Outptr_ ISvcThread **ppThread
        ) PURE;

    // @TODO: There are a number more specific things which should be bridged between these two worlds

};

//
// IDebugHostSymbolsTargetComposition:
//
// An interfaces which bridges the extensibility of the upper edge interfaces (the data model) with those of the
// lower edge (target composition) for symbols.
//
#undef INTERFACE
#define INTERFACE IDebugHostSymbolsTargetComposition
DECLARE_INTERFACE_(IDebugHostSymbolsTargetComposition, IUnknown)
{
    //*************************************************
    // IDebugHostSymbolsTargetComposition:
    //

    // GetTypeForServiceType():
    //
    // From an ISvcSymbolType which is a symbol associated with a given ISvcModule, return the IDebugHostType
    // representing such at the data model level.
    //
    IFACEMETHOD(GetTypeForServiceType)(
        THIS_
        _In_ IDebugServiceManager *pServiceManager,
        _In_ ISvcModule *pModule,
        _In_ ISvcSymbolType *pType,
        _COM_Outptr_ IDebugHostType **ppHostType
        ) PURE;
};

//**************************************************************************
// Public APIs:
//

//
// CreateDataModelManager():
//
// The initial call a host performs to create and initialize the data model.
//
STDAPI CreateDataModelManager(
    _In_ IDebugHost* debugHost,
    _COM_Outptr_ IDataModelManager** manager
    );

//*************************************************
// C++ Specific API Helpers

#if defined(__cplusplus)
}; // extern "C"

namespace Debugger
{
namespace DataModel
{

//
// ConvertException():
//
// Trap and convert all exceptions coming out of a functor to an appropriate HRESULT
//
template<typename FN>
HRESULT ConvertException(const FN& fn)
{
    HRESULT hr;
    try
    {
        hr = fn();
    }
    catch(const std::bad_alloc&)
    {
        hr = E_OUTOFMEMORY;
    }
    catch(...)
    {
        hr = E_FAIL;
    }

    return hr;
}

#ifdef _WRL_CLIENT_H_

//
// BindProperty:
//
// A binder which converts two instance methods on a class to a read/write property accessor.  The class must be IUnknown derived.
// The returned binder will hold reference on the class object.
//
// Usage: BindProperty(this, &MyClass::GetMyProperty, &MyClass::SetMyProperty)
//
template<typename T>
Microsoft::WRL::ComPtr<IModelPropertyAccessor>
BindProperty(
    T* classObject,
    HRESULT (T::* getMethod)(PCWSTR key, IModelObject* contextObject, IModelObject** value),
    HRESULT (T::* setMethod)(PCWSTR key, IModelObject* contextObject, IModelObject** value)
    )
{
    //
    // PropertyAccessor:
    //

    struct PropertyAccessor : Microsoft::WRL::RuntimeClass<Microsoft::WRL::RuntimeClassFlags<Microsoft::WRL::RuntimeClassType::ClassicCom>, IModelPropertyAccessor>
    {
        PropertyAccessor(
            T* classObject,
            HRESULT(T::* getMethod)(PCWSTR, IModelObject*, IModelObject**),
            HRESULT(T::* setMethod)(PCWSTR, IModelObject*, IModelObject**)
            ) :
            _classObject(classObject),
            _getMethod(getMethod),
            _setMethod(setMethod)
        {
        }

        //*************************************************
        // IModelPropertyAccessor:

        STDMETHOD(GetValue)(
            _In_ PCWSTR key,
            _In_opt_ IModelObject* contextObject,
            _COM_Outptr_opt_ IModelObject** value
            )
        {
            return ConvertException( [&](){
                return (_classObject->*_getMethod)(key, contextObject, value);
            });
        }

        STDMETHOD(SetValue)(
            _In_ PCWSTR key,
            _In_opt_ IModelObject* contextObject,
            _In_ IModelObject* value
            )
        {
            return ConvertException( [&](){
                if (_setMethod != nullptr)
                {
                    return (_classObject->*_setMethod)(key, contextObject, value);
                }

                //
                // It's a read-only property.
                //
                return E_NOTIMPL;
            });
        }

        Microsoft::WRL::ComPtr<T> _classObject;
        HRESULT(T::* _getMethod)(PCWSTR, IModelObject*, IModelObject**);
        HRESULT(T::* _setMethod)(PCWSTR, IModelObject*, IModelObject**);

    };

    Microsoft::WRL::ComPtr<PropertyAccessor> spPropertyAccessor = Microsoft::WRL::Make<PropertyAccessor>(classObject, getMethod, setMethod);
    return spPropertyAccessor.Detach();
}

//
// BindProperty:
//
// A binder which converts two lambdas to a read/write property accessor.  The lambdas must hold reference on outer objects they
// reference through a by value capture.
//
// Usage: BindProperty(get_lambda, set_lambda)
//
template<typename TGet, typename TSet>
Microsoft::WRL::ComPtr<IModelPropertyAccessor>
BindProperty(
    const TGet& getFunctor,
    const TSet& setFunctor
    )
{
    //
    // PropertyAccessor:
    //

    struct PropertyAccessor : Microsoft::WRL::RuntimeClass<Microsoft::WRL::RuntimeClassFlags<Microsoft::WRL::RuntimeClassType::ClassicCom>, IModelPropertyAccessor>
    {
        PropertyAccessor(
            const TGet& getFunctor,
            const TSet& setFunctor
            ) :
            _getFunctor(getFunctor),
            _setFunctor(setFunctor)
        {
        }

        //*************************************************
        // IModelPropertyAccessor:

        STDMETHOD(GetValue)(
            _In_ PCWSTR key,
            _In_opt_ IModelObject* contextObject,
            _COM_Outptr_opt_ IModelObject** value
            )
        {
            return ConvertException( [&](){
                return _getFunctor(key, contextObject, value);
            });
        }

        STDMETHOD(SetValue)(
            _In_ PCWSTR key,
            _In_opt_ IModelObject* contextObject,
            _In_ IModelObject* value
            )
        {
            return ConvertException( [&](){
                return _setFunctor(key, contextObject, value);
            });
        }

        TGet _getFunctor;
        TSet _setFunctor;
    };

    Microsoft::WRL::ComPtr<PropertyAccessor> spPropertyAccessor = Microsoft::WRL::Make<PropertyAccessor>(getFunctor, setFunctor);
    return spPropertyAccessor;
}

//
// BindReadOnlyProperty:
//
// A binder which converts one instance method on a class to a read-only property accessor.  The class must be IUnknown derived.
// The returned binder will hold reference on the class object.
//
template<typename T>
Microsoft::WRL::ComPtr<IModelPropertyAccessor>
BindReadOnlyProperty(
    T* classObject,
    HRESULT (T::* getMethod)(PCWSTR key, IModelObject* contextObject, IModelObject** value)
    )
{
    return BindProperty<T>(classObject, getMethod, nullptr);
}

//
// BindReadOnlyProperty:
//
// A binder which converts one lambda to a read-only property accessor.  The lambda must hold reference on outer objects through
// a by value capture.
//
// Usage: BindProperty(get_lambda)
//
template<typename TGet>
Microsoft::WRL::ComPtr<IModelPropertyAccessor>
BindReadOnlyProperty(
    const TGet& getFunctor
    )
{
    //
    // PropertyAccessor:
    //

    struct PropertyAccessor : Microsoft::WRL::RuntimeClass<Microsoft::WRL::RuntimeClassFlags<Microsoft::WRL::RuntimeClassType::ClassicCom>, IModelPropertyAccessor>
    {
        PropertyAccessor(
            const TGet& getFunctor
            ) :
            _getFunctor(getFunctor)
        {
        }

        //*************************************************
        // IModelPropertyAccessor:

        STDMETHOD(GetValue)(
            _In_ PCWSTR key,
            _In_opt_ IModelObject* contextObject,
            _COM_Outptr_opt_ IModelObject** value
            )
        {
            return ConvertException( [&](){
                return _getFunctor(key, contextObject, value);
            });
        }

        STDMETHOD(SetValue)(
            _In_ PCWSTR /*key*/,
            _In_opt_ IModelObject* /*contextObject*/,
            _In_ IModelObject* /*value*/
            )
        {
            return E_NOTIMPL;
        }

        TGet _getFunctor;
    };

    Microsoft::WRL::ComPtr<PropertyAccessor> spPropertyAccessor = Microsoft::WRL::Make<PropertyAccessor>(getFunctor);
    return spPropertyAccessor;
}

#endif // _WRL_CLIENT_H_

} // namespace: DataModel

} // namespace: Debugger

#endif // __cplusplus

#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_PKG_WER) */
#pragma endregion

#endif // __DBGMODEL_H__

