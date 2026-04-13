#pragma once
/* Shim for <oaidl.h> — defines VARIANT and related OLE Automation types. */
#include <objidl.h>

/* -------------------------------------------------------------------------
 * VARIANT — opaque OLE Automation variant type.
 * Full definition not needed for parsing interfaces; forward-declare it.
 * ---------------------------------------------------------------------- */
#ifndef _tagVARIANT_
#define _tagVARIANT_
typedef struct tagVARIANT VARIANT;
struct tagVARIANT {
    unsigned short vt;
    unsigned short wReserved1;
    unsigned short wReserved2;
    unsigned short wReserved3;
    union {
        long long       llVal;
        long            lVal;
        unsigned char   bVal;
        short           iVal;
        float           fltVal;
        double          dblVal;
        short           boolVal;
        long            scode;
        BSTR            bstrVal;
        void           *byref;
    };
};
#endif /* _tagVARIANT_ */
