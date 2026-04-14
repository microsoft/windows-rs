//
// Copyright (c) Microsoft Corporation.  All rights reserved.
//

#pragma once


#ifdef __cplusplus_winrt
#if _MSC_VER >= 1900

// C++/CX mode with a newer compiler.
#define _WINDOWS_NUMERICS_CX_PROJECTION_

#else

// C++/CX mode with an older compiler.
#define _WINDOWS_NUMERICS_INTEROP_NAMESPACE_ Windows::Foundation::Numerics

#endif

// In C++/CX mode, define operators to convert between float2 and Windows::Foundation::Point+Size.
#define _WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_ Windows::Foundation

#elif defined __windows2Efoundation2Enumerics_h__

// Raw COM mode using ABI interop (after including windows.foundation.numerics.h).
#ifdef ____x_Windows_CFoundation_CIClosable_FWD_DEFINED__
#define _WINDOWS_NUMERICS_INTEROP_NAMESPACE_ Windows::Foundation::Numerics
#else
#define _WINDOWS_NUMERICS_INTEROP_NAMESPACE_ ABI::Windows::Foundation::Numerics
#endif

#endif

#define _WINDOWS_NUMERICS_NAMESPACE_ Windows::Foundation::Numerics
#define _WINDOWS_NUMERICS_BEGIN_NAMESPACE_ namespace Windows { namespace Foundation { namespace Numerics
#define _WINDOWS_NUMERICS_END_NAMESPACE_ }}


#include "WindowsNumerics.impl.h"


#undef _WINDOWS_NUMERICS_CX_PROJECTION_
#undef _WINDOWS_NUMERICS_INTEROP_NAMESPACE_
#undef _WINDOWS_NUMERICS_POINT_SIZE_NAMESPACE_
#undef _WINDOWS_NUMERICS_NAMESPACE_
#undef _WINDOWS_NUMERICS_BEGIN_NAMESPACE_
#undef _WINDOWS_NUMERICS_END_NAMESPACE_
