

/* this ALWAYS GENERATED file contains the IIDs and CLSIDs */

/* link this file in with the server and any clients */


 /* File created by MIDL compiler version 8.01.0628 */
/* @@MIDL_FILE_HEADING(  ) */



#ifdef __cplusplus
extern "C"{
#endif 


#include <rpc.h>
#include <rpcndr.h>

#ifdef _MIDL_USE_GUIDDEF_

#ifndef INITGUID
#define INITGUID
#include <guiddef.h>
#undef INITGUID
#else
#include <guiddef.h>
#endif

#define MIDL_DEFINE_GUID(type,name,l,w1,w2,b1,b2,b3,b4,b5,b6,b7,b8) \
        DEFINE_GUID(name,l,w1,w2,b1,b2,b3,b4,b5,b6,b7,b8)

#else // !_MIDL_USE_GUIDDEF_

#ifndef __IID_DEFINED__
#define __IID_DEFINED__

typedef struct _IID
{
    unsigned long x;
    unsigned short s1;
    unsigned short s2;
    unsigned char  c[8];
} IID;

#endif // __IID_DEFINED__

#ifndef CLSID_DEFINED
#define CLSID_DEFINED
typedef IID CLSID;
#endif // CLSID_DEFINED

#define MIDL_DEFINE_GUID(type,name,l,w1,w2,b1,b2,b3,b4,b5,b6,b7,b8) \
        EXTERN_C __declspec(selectany) const type name = {l,w1,w2,{b1,b2,b3,b4,b5,b6,b7,b8}}

#endif // !_MIDL_USE_GUIDDEF_

MIDL_DEFINE_GUID(IID, LIBID_MSINKDIVLib,0x56D04F5D,0x964F,0x4dbf,0x8D,0x23,0xB9,0x79,0x89,0xE5,0x34,0x18);


MIDL_DEFINE_GUID(IID, IID_IInkDivider,0x5DE00405,0xF9A4,0x4651,0xB0,0xC5,0xC3,0x17,0xDE,0xFD,0x58,0xB9);


MIDL_DEFINE_GUID(IID, IID_IInkDivisionResult,0x2DBEC0A7,0x74C7,0x4B38,0x81,0xEB,0xAA,0x8E,0xF0,0xC2,0x49,0x00);


MIDL_DEFINE_GUID(IID, IID_IInkDivisionUnit,0x85AEE342,0x48B0,0x4244,0x9D,0xD5,0x1E,0xD4,0x35,0x41,0x0F,0xAB);


MIDL_DEFINE_GUID(IID, IID_IInkDivisionUnits,0x1BB5DDC2,0x31CC,0x4135,0xAB,0x82,0x2C,0x66,0xC9,0xF0,0x0C,0x41);


MIDL_DEFINE_GUID(CLSID, CLSID_InkDivider,0x8854F6A0,0x4683,0x4AE7,0x91,0x91,0x75,0x2F,0xE6,0x46,0x12,0xC3);

#undef MIDL_DEFINE_GUID

#ifdef __cplusplus
}
#endif



