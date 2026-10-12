typedef unsigned int Count;
enum class Mode : unsigned short { Off = 0, On = 65535 };
typedef enum { Failed = -3, FailedAlias = Failed, Passed = 9 } Status, StatusAlias;
enum Wide : unsigned long long { Maximum = 0xffffffffffffffffULL };
typedef Wide WideAlias;
struct Item { Mode mode; Count count; };
extern "C" {
void Choose(int beta, int alpha);
void Choose(int alpha, int beta);
Status Echo(Status status, const Item *item);
__declspec(dllimport) int ImportCall(Count count);
void Missing(int, int param0);
int Variadic(Count count, ...);
}
