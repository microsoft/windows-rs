

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

MIDL_DEFINE_GUID(IID, IID_ISpellCheckProvider,0x73E976E0,0x8ED4,0x4EB1,0x80,0xD7,0x1B,0xE0,0xA1,0x6B,0x0C,0x38);


MIDL_DEFINE_GUID(IID, IID_IComprehensiveSpellCheckProvider,0x0C58F8DE,0x8E94,0x479E,0x97,0x17,0x70,0xC4,0x2C,0x4A,0xD2,0xC3);


MIDL_DEFINE_GUID(IID, IID_ISpellCheckProviderFactory,0x9F671E11,0x77D6,0x4C92,0xAE,0xFB,0x61,0x52,0x15,0xE3,0xA4,0xBE);

#undef MIDL_DEFINE_GUID

#ifdef __cplusplus
}
#endif



