#include "check.h"
#include <winrt/Composition.h>
#include <winrt/Windows.Foundation.h>

using namespace winrt;
using namespace Composition;

static int32_t derived_instances;

// Overrides `Name` and adds an interface that the base class does not implement.
struct Derived : BaseT<Derived, Windows::Foundation::IStringable> {
    Derived(int32_t value) : BaseT(value) {
        ++derived_instances;
    }

    ~Derived() {
        --derived_instances;
    }

    hstring Name() {
        return L"Cpp:" + BaseT::Name();
    }

    hstring ToString() {
        return L"Stringable";
    }

    // The base class's protected interface is only available to a derived class.
    int32_t ProtectedDoubled() {
        return Doubled();
    }
};

// Derives without overriding anything.
struct Plain : BaseT<Plain> {
    Plain() : BaseT(0) {}
};

void test() {
    check(Base::Instances() == 0);

    {
        Base base(5);
        check(base.Value() == 5);
        check(base.Describe() == L"Base(5)");
        check(base.Stringify() == L"");
        check(Base::Instances() == 1);
    }

    check(Base::Instances() == 0);

    {
        auto derived = make_self<Derived>(7);
        Base base = *derived;

        // The base calls the override through the outer object.
        check(base.Value() == 7);
        check(base.Describe() == L"Cpp:Base(7)");
        check(base.Stringify() == L"Stringable");
        check(derived->ProtectedDoubled() == 14);

        // QueryInterface reaches the derived and base interfaces from either side, with one
        // identity.
        check(base.as<Windows::Foundation::IStringable>().ToString() == L"Stringable");
        check(base.as<IBaseOverrides>().Name() == L"Cpp:Base");
        check(base.as<Windows::Foundation::IStringable>().as<IBase>().Value() == 7);
        check(base.as<Windows::Foundation::IUnknown>() == derived.as<Windows::Foundation::IUnknown>());

        check(Base::Instances() == 1);
        check(derived_instances == 1);
    }

    check(Base::Instances() == 0);
    check(derived_instances == 0);

    {
        Base base = make<Plain>();
        check(base.Describe() == L"Base(0)");
        check(base.Stringify() == L"");
        check(base.as<IBaseOverrides>().Name() == L"Base");
        check(!base.try_as<Windows::Foundation::IStringable>());
        check(Base::Instances() == 1);
    }

    check(Base::Instances() == 0);
}
