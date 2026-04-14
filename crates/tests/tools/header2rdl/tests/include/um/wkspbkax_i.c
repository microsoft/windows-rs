

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

MIDL_DEFINE_GUID(IID, IID_IWorkspaceBrokerAx,0x2F93474C,0x2ED7,0x4A68,0xAF,0xD1,0xD3,0xC9,0x78,0x1F,0x34,0x1E);


MIDL_DEFINE_GUID(IID, IID_IWorkspaceBrokerAx2,0x57D42F01,0xD762,0x4680,0x98,0x93,0x32,0x6B,0x08,0xF2,0x26,0x52);


MIDL_DEFINE_GUID(IID, LIBID_wkspbkaxLib,0xA47EDEDD,0x2921,0x4E5A,0x9B,0x91,0x13,0xC9,0x53,0xEC,0x7F,0xE3);


MIDL_DEFINE_GUID(IID, DIID__IWorkspaceBrokerAxEvents,0x7A3D0ADC,0x0CC4,0x473D,0xB3,0x27,0xD3,0x78,0x86,0x30,0x8F,0x28);


MIDL_DEFINE_GUID(CLSID, CLSID_WorkspaceBrokerAx,0xCD70A734,0xB6DB,0x4588,0x98,0x13,0xFF,0x2E,0x37,0xA4,0x66,0x1F);

#undef MIDL_DEFINE_GUID

#ifdef __cplusplus
}
#endif



