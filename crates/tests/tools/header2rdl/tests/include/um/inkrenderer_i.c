

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

MIDL_DEFINE_GUID(IID, IID_IInkD2DRenderer,0x407fb1de,0xf85a,0x4150,0x97,0xcf,0xb7,0xfb,0x27,0x4f,0xb4,0xf8);


MIDL_DEFINE_GUID(IID, IID_IInkD2DRenderer2,0x0a95dcd9,0x4578,0x4b71,0xb2,0x0b,0xbf,0x66,0x4d,0x4b,0xfe,0xee);


MIDL_DEFINE_GUID(IID, LIBID_InkD2DRendererLib,0x390d0ab0,0x19e2,0x46bb,0x86,0x2e,0xb0,0x9f,0x3c,0xdc,0xf8,0xb9);


MIDL_DEFINE_GUID(CLSID, CLSID_InkD2DRenderer,0x4044e60c,0x7b01,0x4671,0xa9,0x7c,0x04,0xe0,0x21,0x0a,0x07,0xa5);

#undef MIDL_DEFINE_GUID

#ifdef __cplusplus
}
#endif



