

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

MIDL_DEFINE_GUID(IID, LIBID_INKEDLib,0x8405D0DF,0x9FDD,0x4829,0xAE,0xAD,0x8E,0x2B,0x0A,0x18,0xFE,0xA4);


MIDL_DEFINE_GUID(IID, IID_IInkEdit,0xF2127A19,0xFBFB,0x4AED,0x84,0x64,0x3F,0x36,0xD7,0x8C,0xFE,0xFB);


MIDL_DEFINE_GUID(IID, DIID__IInkEditEvents,0xE3B0B797,0xA72E,0x46DB,0xA0,0xD7,0x6C,0x9E,0xBA,0x8E,0x9B,0xBC);


MIDL_DEFINE_GUID(CLSID, CLSID_InkEdit,0xE5CA59F5,0x57C4,0x4DD8,0x9B,0xD6,0x1D,0xEE,0xED,0xD2,0x7A,0xF4);

#undef MIDL_DEFINE_GUID

#ifdef __cplusplus
}
#endif



