struct Data { VALUE value; };
struct Packet { Data* value; };
typedef void (*Callback)(Packet* value);
extern "C" void Use(Packet* value);
extern const Packet ITEM;
