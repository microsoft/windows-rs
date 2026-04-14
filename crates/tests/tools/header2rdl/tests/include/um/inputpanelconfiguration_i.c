

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

MIDL_DEFINE_GUID(IID, IID_IInputPanelConfiguration,0x41C81592,0x514C,0x48BD,0xA2,0x2E,0xE6,0xAF,0x63,0x85,0x21,0xA6);


MIDL_DEFINE_GUID(IID, IID_IInputPanelInvocationConfiguration,0xA213F136,0x3B45,0x4362,0xA3,0x32,0xEF,0xB6,0x54,0x7C,0xD4,0x32);


MIDL_DEFINE_GUID(IID, LIBID_InputPanelConfigurationLib,0x82E4F0B2,0x5440,0x42E4,0x8E,0xD9,0xA9,0x15,0xD1,0x21,0x6C,0x79);


MIDL_DEFINE_GUID(CLSID, CLSID_InputPanelConfiguration,0x2853ADD3,0xF096,0x4C63,0xA7,0x8F,0x7F,0xA3,0xEA,0x83,0x7F,0xB7);

#undef MIDL_DEFINE_GUID

#ifdef __cplusplus
}
#endif



