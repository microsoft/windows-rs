//*********************************************************************
//*                  Microsoft Windows                               **
//*            Copyright(c) Microsoft Corp., 1996-1997               **
//*********************************************************************

//;begin_internal
/***********************************************************************************************

  This is a distributed SDK component - do not put any #includes or other directives that rely
  upon files not dropped. If in doubt - build iedev

  If you add comments please include either ;BUGBUG at the beginning of a single line OR
  enclose in a ;begin_internal, ;end_internal block - such as this one!

 ***********************************************************************************************/
//;end_internal

//;begin_internal
#ifndef __XMLDSODID_H__
#define __XMLDSODID_H__
//;end_internal

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)

#define  DISPID_XMLDSO                       0x00010000
#define  DISPID_XMLDSO_DOCUMENT              DISPID_XMLDSO  +  1
#define  DISPID_XMLDSO_JAVADSOCOMPATIBLE     DISPID_XMLDSO_DOCUMENT  +  1


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion


//;begin_internal
#endif // __XMLDSODID_H__
//;end_internal
