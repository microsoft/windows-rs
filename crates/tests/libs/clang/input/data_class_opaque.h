class PrivateData
{
    int value;
};

class ProtectedData
{
protected:
    int value;
};

class MethodData
{
public:
    int value;
    void Reset();
};

class ConstructorData
{
public:
    ConstructorData();
    int value;
};

class DestructorData
{
public:
    ~DestructorData();
    int value;
};

class NonTrivialMember
{
public:
    NonTrivialMember();
    int value;
};

class NonPodWrapper
{
public:
    NonTrivialMember value;
};

class BaseData
{
public:
    int value;
};

class DerivedData : public BaseData
{
public:
    int other;
};

class Point
{
public:
    Point();
    Point(int x, int y);
    bool Equals(const Point& other);
    int x;
    int y;
};

extern "C" void UsePrivate(PrivateData* value);
extern "C" void UseProtected(ProtectedData* value);
extern "C" void UseMethod(MethodData* value);
extern "C" void UseConstructor(ConstructorData* value);
extern "C" void UseDestructor(DestructorData* value);
extern "C" void UseNonPodWrapper(NonPodWrapper* value);
extern "C" void UseDerived(DerivedData* value);
extern "C" void UsePoint(Point* value);
