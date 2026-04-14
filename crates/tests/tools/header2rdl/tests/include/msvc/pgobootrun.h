#if defined(_WCHAR_T_DEFINED)
typedef void (__cdecl *POGOAUTOSWEEPPROCW)(const wchar_t *);
#else
typedef void (__cdecl *POGOAUTOSWEEPPROCW)(const unsigned short *);
#endif
typedef void (__cdecl *POGOAUTOSWEEPPROCA)(const char *);

#ifdef __cplusplus
extern "C"
#else
extern
#endif
POGOAUTOSWEEPPROCW PogoAutoSweepW;
#ifdef __cplusplus
extern "C"
#else
extern
#endif
POGOAUTOSWEEPPROCA PogoAutoSweepA;

#ifdef UNICODE
#define PgoAutoSweep PogoAutoSweepW
#else
#define PgoAutoSweep PogoAutoSweepA
#endif
