struct Point {
    float x, y;
    Point() : x(0), y(0) {}
    Point(const Point& other) : x(other.x), y(other.y) {}
};
struct Constructed {
    int value;
    Constructed() : value(7) {}
    operator int() const { return value; }
};
class Empty {};
struct EmptyStruct {};
class DerivedEmpty : public Empty {};
class Forward;
class Hidden {
#ifdef PROTECTED_STORAGE
protected:
#endif
    int value;
public:
    Hidden() : value(0) {}
    friend class Empty;
};
class Resource {
public:
    int value;
    ~Resource();
};
inline Resource::~Resource() {}
typedef Hidden Handle;
struct Points { Point values[2]; };
struct InvalidStorage { Handle value; };
struct InvalidArray { Handle values[2]; };
extern "C" void ReadPoints(const Points* points);
extern "C" void UseConstructed(Constructed value);
extern "C" void UseHandles(Empty*, EmptyStruct*, DerivedEmpty*, Forward*, Handle*, Resource*);
extern "C" Point ReturnPoint();
extern "C" Constructed ReturnConstructed();
extern "C" void CopyPoint(Point value);
extern "C" Hidden ReturnHidden();
extern "C" void CopyEmpty(Empty value);
extern "C" void CopyResource(Resource value);
extern "C" void UseInvalidStorage(InvalidStorage* value);
extern "C" void UseInvalidArray(InvalidArray* value);
