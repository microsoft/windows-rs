/****************************************************************************
* Copyright (c) Microsoft Corporation. All rights reserved.
****************************************************************************/
#ifdef MIDL_PASS
cpp_quote("#include <winapifamily.h>")

#pragma region Desktop Family
cpp_quote("#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)")

#include <devpropdef.h>


cpp_quote("#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */")
#pragma endregion

#endif // MIDL_PASS
