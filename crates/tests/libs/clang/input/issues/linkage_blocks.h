extern "C" {
struct Point { int x; int y; };
enum Mode { ModeA, ModeB };
typedef struct Point* PPoint;
void __stdcall UsePoint(PPoint point, enum Mode mode);
}
void __stdcall CppOnly(int value);
