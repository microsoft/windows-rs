#include <winrt/WeakReferences.h>
#include "check.h"

using namespace winrt;
using namespace WeakReferences;

struct Observer : implements<Observer, IObserver>
{
    int32_t* m_value;

    Observer(int32_t* value) : m_value(value)
    {
    }

    void Notify(int32_t value)
    {
        *m_value = value;
    }
};

void test()
{
    Widget widget;
    int32_t value = 0;

    IObserver observer = make<Observer>(&value);
    widget.Observe(observer);
    check(widget.Notify(1));
    check(value == 1);

    observer = nullptr;
    check(!widget.Notify(2));
    check(value == 1);

    weak_ref<Widget> weak = make_weak(widget);
    check(weak.get() == widget);
    widget = nullptr;
    check(!weak.get());
}