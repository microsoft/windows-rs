

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

MIDL_DEFINE_GUID(IID, IID_ISpellingError,0xB7C82D61,0xFBE8,0x4B47,0x9B,0x27,0x6C,0x0D,0x2E,0x0D,0xE0,0xA3);


MIDL_DEFINE_GUID(IID, IID_IEnumSpellingError,0x803E3BD4,0x2828,0x4410,0x82,0x90,0x41,0x8D,0x1D,0x73,0xC7,0x62);


MIDL_DEFINE_GUID(IID, IID_IOptionDescription,0x432E5F85,0x35CF,0x4606,0xA8,0x01,0x6F,0x70,0x27,0x7E,0x1D,0x7A);


MIDL_DEFINE_GUID(IID, IID_ISpellCheckerChangedEventHandler,0x0B83A5B0,0x792F,0x4EAB,0x97,0x99,0xAC,0xF5,0x2C,0x5E,0xD0,0x8A);


MIDL_DEFINE_GUID(IID, IID_ISpellChecker,0xB6FD0B71,0xE2BC,0x4653,0x8D,0x05,0xF1,0x97,0xE4,0x12,0x77,0x0B);


MIDL_DEFINE_GUID(IID, IID_ISpellChecker2,0xE7ED1C71,0x87F7,0x4378,0xA8,0x40,0xC9,0x20,0x0D,0xAC,0xEE,0x47);


MIDL_DEFINE_GUID(IID, IID_ISpellCheckerFactory,0x8E018A9D,0x2415,0x4677,0xBF,0x08,0x79,0x4E,0xA6,0x1F,0x94,0xBB);


MIDL_DEFINE_GUID(IID, IID_IUserDictionariesRegistrar,0xAA176B85,0x0E12,0x4844,0x8E,0x1A,0xEE,0xF1,0xDA,0x77,0xF5,0x86);


MIDL_DEFINE_GUID(IID, LIBID_MsSpellCheckLib,0x4A250E01,0x61EA,0x400B,0xA2,0x7D,0xBF,0x37,0x44,0xBC,0xC9,0xF5);


MIDL_DEFINE_GUID(CLSID, CLSID_SpellCheckerFactory,0x7AB36653,0x1796,0x484B,0xBD,0xFA,0xE7,0x4F,0x1D,0xB7,0xC1,0xDC);

#undef MIDL_DEFINE_GUID

#ifdef __cplusplus
}
#endif



