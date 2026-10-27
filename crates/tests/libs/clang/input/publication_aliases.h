#pragma once
struct A { int value; };
typedef A First;
#ifdef DIRECT_SECOND
typedef A Second;
#else
typedef First Second;
#endif
typedef Second* Pointer;
struct User { First first; Second second; };
