

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

MIDL_DEFINE_GUID(IID, IID_IInkCommitRequestHandler,0xfabea3fc,0xb108,0x45b6,0xa9,0xfc,0x8d,0x08,0xfa,0x9f,0x85,0xcf);


MIDL_DEFINE_GUID(IID, IID_IInkPresenterDesktop,0x73f3c0d9,0x2e8b,0x48f3,0x89,0x5e,0x20,0xcb,0xd2,0x7b,0x72,0x3b);


MIDL_DEFINE_GUID(IID, IID_IInkHostWorkItem,0xccda0a9a,0x1b78,0x4632,0xbb,0x96,0x97,0x80,0x06,0x62,0xe2,0x6c);


MIDL_DEFINE_GUID(IID, IID_IInkDesktopHost,0x4ce7d875,0xa981,0x4140,0xa1,0xff,0xad,0x93,0x25,0x8e,0x8d,0x59);


MIDL_DEFINE_GUID(IID, LIBID_InkDesktopHostLib,0x2aef0967,0xc833,0x4f38,0x91,0xf3,0x16,0xe6,0x7d,0x55,0xd7,0x17);


MIDL_DEFINE_GUID(CLSID, CLSID_InkDesktopHost,0x062584a6,0xf830,0x4bdc,0xa4,0xd2,0x0a,0x10,0xab,0x06,0x2b,0x1d);

#undef MIDL_DEFINE_GUID

#ifdef __cplusplus
}
#endif



