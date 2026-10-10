#ifdef PROFILE_PRIMARY
#define DIRECTION "_In_"
#else
#define DIRECTION "_Out_"
#endif
#ifdef PROFILE_UNANNOTATED
#define CONTRACT
#else
#define CONTRACT __attribute__((annotate(DIRECTION)))
#endif
typedef void (__stdcall *Callback)(int* CONTRACT value);
struct Contract { Callback callback; };
extern "C" void UseContract(Contract* value);
