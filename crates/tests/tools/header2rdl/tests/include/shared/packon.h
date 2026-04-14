/*++

Copyright (c) 1990,91  Microsoft Corporation

Module Name:

    packon.h

Abstract:

    This file turns packing of structures on.  (That is, it disables
    automatic alignment of structure fields.)  An include file is needed
    because various compilers do this in different ways.

    The file packoff.h is the complement to this file.

Revision History:

    15-Apr-1991 
        Created lint-able variant.
--*/

#if ! (defined(lint) || defined(_lint))
#if ( _MSC_VER >= 800 )
#pragma warning(disable:4103)
#endif
#pragma pack(1)                 // x86, MS compiler; MIPS, MIPS compiler
#endif // ! (defined(lint) || defined(_lint))
