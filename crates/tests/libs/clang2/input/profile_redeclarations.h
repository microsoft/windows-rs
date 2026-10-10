typedef unsigned long DWORD;
typedef unsigned long ULONG;
typedef DWORD ULONG;
struct ProfileRecord { ULONG value; };
extern "C" void ProfileUse(ProfileRecord* value);
