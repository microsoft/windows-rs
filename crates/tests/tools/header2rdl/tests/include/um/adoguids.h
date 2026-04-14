/********************************************************
*                                                       *
*   Copyright (C) Microsoft. All rights reserved.       *
*                                                       *
********************************************************/

//-----------------------------------------------------------------------------
// File:		adoguids.h
//
// Contents:	ADO GUIDs macros definition
//
// Comments:  This file can be included multiple times as long as INITGUIDS is not defined
//				It can only be included once after an INITGUIDS definition.
//
//-----------------------------------------------------------------------------


#define STRING_GUID(l,w1,w2,b1,b2,b3,b4,b5,b6,b7,b8) l##-##w1##-##w2##-##b1##b2##-##b3##b4##b5##b6##b7##b8

#if defined(__midl) || defined(GEN_MIDL)
#define GUID_BUILDER(n,l,w1,w2,b1,b2,b3,b4,b5,b6,b7,b8) STRING_GUID(l,w1,w2,b1,b2,b3,b4,b5,b6,b7,b8)
#else
#define GUID_BUILDER(n,l,w1,w2,b1,b2,b3,b4,b5,b6,b7,b8) DEFINE_GUID(n,0x##l,0x##w1,0x##w2,0x##b1,0x##b2,0x##b3,0x##b4,0x##b5,0x##b6,0x##b7,0x##b8)
#define IMMEDIATE_GUID_USE
#endif

#define INCLUDING_ADOGUIDS
#include <adogpool.h>
#undef  INCLUDING_ADOGUIDS

#undef IMMEDIATE_GUID_USE
