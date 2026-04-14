

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

MIDL_DEFINE_GUID(IID, IID_IMathInputControl,0xEBA615AA,0xFAC6,0x4738,0xBA,0x5F,0xFF,0x09,0xE9,0xFE,0x47,0x3E);


MIDL_DEFINE_GUID(IID, LIBID_micautLib,0x928CEF0C,0x5A84,0x48AC,0xBF,0x37,0xC5,0xC2,0x10,0x39,0xB8,0x3A);


MIDL_DEFINE_GUID(IID, DIID__IMathInputControlEvents,0x683336B5,0xA47D,0x4358,0x96,0xF9,0x87,0x5A,0x47,0x2A,0xE7,0x0A);


MIDL_DEFINE_GUID(CLSID, CLSID_MathInputControl,0xC561816C,0x14D8,0x4090,0x83,0x0C,0x98,0xD9,0x94,0xB2,0x1C,0x7B);

#undef MIDL_DEFINE_GUID

#ifdef __cplusplus
}
#endif



