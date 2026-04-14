

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

MIDL_DEFINE_GUID(IID, IID_ITSGAuthorizeConnectionSink,0xc27ece33,0x7781,0x4318,0x98,0xef,0x1c,0xf2,0xda,0x7b,0x70,0x05);


MIDL_DEFINE_GUID(IID, IID_ITSGAuthorizeResourceSink,0xfeddfcd4,0xfa12,0x4435,0xae,0x55,0x7a,0xd1,0xa9,0x77,0x9a,0xf7);


MIDL_DEFINE_GUID(IID, IID_ITSGPolicyEngine,0x8bc24f08,0x6223,0x42f4,0xa5,0xb4,0x8e,0x37,0xcd,0x13,0x5b,0xbd);


MIDL_DEFINE_GUID(IID, IID_ITSGAccountingEngine,0x4ce2a0c9,0xe874,0x4f1a,0x86,0xf4,0x06,0xbb,0xb9,0x11,0x53,0x38);

#undef MIDL_DEFINE_GUID

#ifdef __cplusplus
}
#endif



