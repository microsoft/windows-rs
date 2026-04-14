

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

MIDL_DEFINE_GUID(IID, IID_ITipAutoCompleteProvider,0x7C6CF46D,0x8404,0x46b9,0xAD,0x33,0xF5,0xB6,0x03,0x6D,0x40,0x07);


MIDL_DEFINE_GUID(IID, IID_ITipAutoCompleteClient,0x5E078E03,0x8265,0x4bbe,0x94,0x87,0xD2,0x42,0xED,0xBE,0xF9,0x10);


MIDL_DEFINE_GUID(IID, LIBID_TipAutoCompleteClientLib,0x6B56A75E,0xD676,0x4864,0x8B,0x36,0xA1,0x9B,0x89,0x73,0xDB,0x13);


MIDL_DEFINE_GUID(CLSID, CLSID_TipAutoCompleteClient,0x807C1E6C,0x1D00,0x453f,0xB9,0x20,0xB6,0x1B,0xB7,0xCD,0xD9,0x97);

#undef MIDL_DEFINE_GUID

#ifdef __cplusplus
}
#endif



