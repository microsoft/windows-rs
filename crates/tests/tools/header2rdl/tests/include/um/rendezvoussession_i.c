

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

MIDL_DEFINE_GUID(IID, LIBID_RendezvousSessionLib,0xEFD856A4,0x5A85,0x4A1B,0xAD,0xD5,0x2E,0xAD,0xAC,0xE6,0xF6,0xA2);


MIDL_DEFINE_GUID(IID, IID_IRendezvousSession,0x9BA4B1DD,0x8B0C,0x48B7,0x9E,0x7C,0x2F,0x25,0x85,0x7C,0x8D,0xF5);


MIDL_DEFINE_GUID(IID, DIID_DRendezvousSessionEvents,0x3FA19CF8,0x64C4,0x4F53,0xAE,0x60,0x63,0x5B,0x38,0x06,0xEC,0xA6);


MIDL_DEFINE_GUID(IID, IID_IRendezvousApplication,0x4F4D070B,0xA275,0x49FB,0xB1,0x0D,0x8E,0xC2,0x63,0x87,0xB5,0x0D);


MIDL_DEFINE_GUID(CLSID, CLSID_RendezvousApplication,0x0B7E019A,0xB5DE,0x47fa,0x89,0x66,0x90,0x82,0xF8,0x2F,0xB1,0x92);

#undef MIDL_DEFINE_GUID

#ifdef __cplusplus
}
#endif



