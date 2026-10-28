struct Plain { int value; };
struct Assigned {
    int value;
#ifndef PLAIN_ASSIGNMENT
    Assigned& operator=(const Assigned& other) { value = other.value; return *this; }
#endif
};
struct Deleted {
    int value;
    Deleted& operator=(const Deleted&) = delete;
};
typedef Assigned AssignedAlias;
struct ContainsAssigned { Assigned values[2]; };
extern "C" Plain EchoPlain(Plain value);
extern "C" void UseAssigned(Assigned value);
extern "C" void PointToAssigned(Assigned* value);
extern "C" Assigned ReturnAssigned();
extern "C" AssignedAlias ReturnAlias();
extern "C" ContainsAssigned ReturnContainer();
extern "C" Deleted ReturnDeleted();
typedef Assigned (__cdecl *ReturnCallback)();
extern "C" void SetCallback(ReturnCallback callback);
