extern "C" const int Changed = 7;
extern "C" const int Matching = 9;
extern "C" const int Failed = 11;
extern "C" const int Linked;
extern "C" const int Other;
extern "C" const int DeclarationOnly;
extern "C" const int Invalid;
extern "C" __declspec(align(16)) const int Unpublishable = 31;
extern "C" int Runtime();

#define Changed 8
#define Matching 9
#define Failed Runtime()
#define Linked Other
#define MacroOnly 13
#define DeclarationOnly 23
#define Invalid 29
#define Unpublishable 37
