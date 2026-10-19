#include <winrt/Constructors.h>
#include "check.h"

using namespace winrt;
using namespace Constructors;

void test()
{
    check(Widget().Value() == 0);
    check(Widget(123).Value() == 123);

    check(Base().Value() == 0);
    check(Base(456).Value() == 456);

    // Each constructor releases the inner object that the factory returns.
    {
        Base base;
        check(Base::Instances() == 1);
    }
    check(Base::Instances() == 0);
}
