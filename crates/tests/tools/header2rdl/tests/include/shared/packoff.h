/*++

Copyright (c) 1990,91  Microsoft Corporation

Module Name:

    packoff.h

Abstract:

    This file turns packing of structures off.  (That is, it enables
    automatic alignment of structure fields.)  An include file is needed
    because various compilers do this in different ways.

    packoff.h is the complement to packon.h.  An inclusion of packoff.h
    MUST ALWAYS be preceded by an inclusion of packon.h, in one-to-one
    correspondence.

Revision History:

    15-Apr-1991 
        Created lint-able variant.
--*/

#if ! (defined(lint) || defined(_lint))
#if ( _MSC_VER >= 800 )
#pragma warning(disable:4103)
#endif
#pragma pack()
#endif // ! (defined(lint) || defined(_lint))
