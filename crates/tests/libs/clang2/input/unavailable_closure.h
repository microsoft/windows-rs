typedef __int128 Wide;
enum TooWide : unsigned __int128 { Large = (unsigned __int128)1 << 100 };
struct Node { Node* next; TooWide* value; };
typedef Node* NodePointer;
extern "C" void Invalid(NodePointer pointer);
void Overloaded(TooWide* value);
void Overloaded(int value);
struct Packet { int value; };
extern "C" void Use(Packet* packet);
