// Copyright (C) Microsoft Corporation. All rights reserved.

#pragma once

#pragma region Desktop Family or OneCore Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)

#ifndef NETCX_ADAPTER_2
#error include netadaptercx.h
#endif

EXTERN_C_START

struct _NET_PACKET;
typedef struct _NET_PACKET NET_PACKET;

struct _NET_FRAGMENT;
typedef struct _NET_FRAGMENT NET_FRAGMENT;

struct _NET_FRAGMENT_RETURN_CONTEXT;
typedef struct _NET_FRAGMENT_RETURN_CONTEXT NET_FRAGMENT_RETURN_CONTEXT;

#pragma warning(push)
#pragma warning(disable:4201) // 'nonstandard extension used: nameless struct/union'

// This structure holds opaque elements in a ring buffer.
typedef struct DECLSPEC_CACHEALIGN _NET_RING
{
    // Reserved for the operating system.
    UINT16 OSReserved1;

    // Number of bytes from one element to the next.
    // Use  ((BYTE*)p + ElementStride)  to obtain the address of the next element.
    UINT16 ElementStride;

    // Number of elements in the ring buffer.  Is always a power of 2, greater than 1.
    UINT32 NumberOfElements;

    // A mask that can be used to efficiently clamp an index to [0, NumberOfElements).
    // Use the identity:  (x % NumberOfElements) == (x & ElementIndexMask)
    UINT32 ElementIndexMask;

    // Index of the end of the range of elements owned by the NetAdapter.
    UINT32 EndIndex;

    union {
        UINT32 OSReserved0;
        // Reserved for the operating system.
        void * OSReserved2[4];
    } DUMMYUNIONNAME;

    // Index of the first element owned by the NetAdapter.
    UINT32 BeginIndex;

    // The NIC driver may optionally use this to track which elements have been consumed.
    UINT32 NextIndex;

    // Available for the NIC driver to use in any way.
    void * Scratch;

    // Storage for element 0 in the ring buffer.
    // Use NetRingGetElementAtIndex to access elements.
    DECLSPEC_CACHEALIGN
    _Field_size_(NumberOfElements * ElementStride)
    unsigned char
        Buffer[ANYSIZE_ARRAY];

} NET_RING;

C_ASSERT(FIELD_OFFSET(NET_RING, Buffer) == SYSTEM_CACHE_ALIGNMENT_SIZE);

#pragma warning(pop)

/*++

Routine Description:

    Returns the element at the specified index in the Ring

Arguments:

    Ring - The Ring Buffer to access

    Index - The element index.  Must be in the range [0, Ring->NumberOfElements).

Return Value:

    The specified element

--*/

inline
void *
NetRingGetElementAtIndex(
    _In_ NET_RING const * Ring,
    _In_ UINT32 Index
)
{
    return (void *)(Ring->Buffer + (SIZE_T)Index * Ring->ElementStride);
}

inline
UINT32
NetRingAdvanceIndex(
    _In_ NET_RING const * Ring,
    _In_ UINT32 Index,
    _In_ INT32 Distance
)
{
    return (Index + Distance) & Ring->ElementIndexMask;
}

/*++

Routine Description:

    Returns the next index, wrapping around if necessary

Arguments:

    Ring - The Ring Buffer to access

    Index - The starting index

Return Value:

    The next index after the starting index

--*/
inline
UINT32
NetRingIncrementIndex(
    _In_ NET_RING const * Ring,
    _In_ UINT32 Index
)
{
    return NetRingAdvanceIndex(Ring, Index, 1);
}

inline
UINT32
NetRingGetRangeCount(
    _In_ NET_RING const * Ring,
    _In_ UINT32 StartIndex,
    _In_ UINT32 EndIndex
)

/*++

Routine Description:

    Calculates the number of elements contained in a range

    Examples, assuming the ring has 8 elements total:
        - the number of elements in the range [1, 4) is 3.
        - the number of elements in the range [4, 1) is 5.
        - the number of elements in the range [7, 7) is 0.

Arguments:

    Ring - The Ring Buffer to access

    StartIndex - The inclusive start of the range to measure

    EndIndex - The exclusive end of the range to measure

Return Value:

    The number of elements in the given range

--*/

{

    NT_ASSERT(StartIndex < Ring->NumberOfElements);
    NT_ASSERT(EndIndex < Ring->NumberOfElements);

    return (EndIndex - StartIndex) & Ring->ElementIndexMask;
}

inline
NET_PACKET *
NetRingGetPacketAtIndex(
    NET_RING const * Ring,
    UINT32 Index
)
{
    return (NET_PACKET *)NetRingGetElementAtIndex(Ring, Index);
}

inline
NET_FRAGMENT *
NetRingGetFragmentAtIndex(
    NET_RING const * Ring,
    UINT32 Index
)
{
    return (NET_FRAGMENT *)NetRingGetElementAtIndex(Ring, Index);
}

inline
NET_FRAGMENT_RETURN_CONTEXT *
NetRingGetFragmentReturnContextAtIndex(
    NET_RING const * Ring,
    UINT32 Index
)
{
    return (NET_FRAGMENT_RETURN_CONTEXT *)NetRingGetElementAtIndex(Ring, Index);
}

EXTERN_C_END

#endif // WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP | WINAPI_PARTITION_SYSTEM)
#pragma endregion

