/*++

Copyright (C) 1999 Microsoft Corporation

Module Name:

    POLARITY.H

Abstract:

    properly map __declspec( dllexport ) or import

History:

--*/

#if _MSC_VER > 1000
#pragma once
#endif

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

// If we are building the DLL then define the 
// class as exported otherwise as imported
// ============================================
#ifndef POLARITY_HEADERFILE_IS_INCLUDED
#define POLARITY_HEADERFILE_IS_INCLUDED
//#pragma message( "Including Polarity.h..." )

 #ifdef USE_POLARITY
  #ifdef BUILDING_DLL
//   #pragma message( "Building static library or DLL..." )
   #define POLARITY __declspec( dllexport )
  #else 
//   #pragma message( "Building Provider..." )
   #define POLARITY __declspec( dllimport )
  #endif
 #else
  #define POLARITY
//  #pragma message( "NO Polarity...")
 #endif
#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion