#ifdef SDK_SAL
#include <sal.h>
#else
#define SAL(name) __attribute__((annotate("windows_clang:sal:2:" name "\n()\n()")))
#define _Pre_ SAL("_Pre_")
#define _Post_ SAL("_Post_")
#define _Valid_ SAL("_Valid_")
#define _In_ SAL("_In_")
#define _Notref_ SAL("_Notref_")
#define _Null_terminated_ SAL("_Null_terminated_")
#define _NullNull_terminated_ SAL("_NullNull_terminated_")
#define _When_(condition, annotations) __attribute__((annotate("windows_clang:sal:2:_When_\n(" #condition "," #annotations ")\n(" #condition "," #annotations ")")))
#define _Group_(annotations) __attribute__((annotate("windows_clang:sal:2:_Group_\n(" #annotations ")\n(" #annotations ")")))
#define _At_(target, annotations) __attribute__((annotate("windows_clang:sal:2:_At_\n(" #target "," #annotations ")\n(" #target "," #annotations ")")))
#endif
#ifndef FIRST_PHASE
#define FIRST_PHASE _Pre_
#endif
#ifndef SECOND_PHASE
#define SECOND_PHASE _Post_
#endif
extern "C" void InputSingle(_Pre_ _Null_terminated_ const char* value);
extern "C" void OutputSingle(_Post_ _Null_terminated_ char* value);
extern "C" void InputMulti(_Pre_ _NullNull_terminated_ const unsigned short* value);
extern "C" void OutputMulti(_Post_ _NullNull_terminated_ unsigned short* value);
typedef unsigned short WCHAR;
extern "C" void MultiString(_Post_ _NullNull_terminated_ WCHAR* value);
extern "C" void Both(FIRST_PHASE _NullNull_terminated_ SECOND_PHASE _NullNull_terminated_ char* value);
extern "C" void Mixed(FIRST_PHASE _Null_terminated_ SECOND_PHASE _NullNull_terminated_ char* value);
extern "C" void OtherProperty(_Post_ _Valid_ _NullNull_terminated_ char* value);
extern "C" void Conditional(_When_(enabled, _Post_ _NullNull_terminated_) char* value, int enabled);
extern "C" void Grouped(_Post_ _Group_(_NullNull_terminated_ _Valid_) char* value);
extern "C" void Indirect(_At_(*value, _Post_ _NullNull_terminated_) char** value);
extern "C" void Modified(_Post_ _Notref_ _NullNull_terminated_ char* value);
extern "C" void Bare(_NullNull_terminated_ char* value);
extern "C" void QualifiedContract(_Post_ _In_ char* value);
typedef FIRST_PHASE _Null_terminated_ SECOND_PHASE _NullNull_terminated_ char* ScopedAlias;
#ifdef INVALID_BUFFER
extern "C" void Invalid(_Post_ _NullNull_terminated_ int value);
#endif
typedef void (*MultiCallback)(_Post_ _NullNull_terminated_ unsigned short* value);
struct __declspec(uuid("11111111-2222-3333-4444-555555555555")) IMultiString {
    virtual void __stdcall Get(_Post_ _NullNull_terminated_ unsigned short* value) = 0;
};
