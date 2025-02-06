#include <windows.h>
#include "winrt/Test.h"
#include "winrt/Windows.Foundation.Collections.h"

using namespace winrt;
using namespace Windows::Foundation::Collections;
using namespace winrt::Test;

struct Implementation : implements<Implementation, ITest>
{
    void TestIterable(IIterable<int> const& collection, array_view<const int> values) {
        std::vector<int> buffer(values.size());
        collection.First().GetMany(buffer);
        auto span1 = std::span(buffer);
        auto span2 = std::span(values);

        // For some reason this doesn't work... assert(span1 == span2);
        assert(span1.size() == span2.size());
        assert(std::equal(span1.begin(), span1.end(), span2.begin()));
    }
    
    IIterable<int> GetIterable(array_view<const int> values) {
        return single_threaded_vector(std::vector(values.begin(), values.end()));
    }

    IMapView<int, IVectorView<int>> GetMapView(array_view<const int> values) {
        std::map<int, IVectorView<int>> map;

        for (auto value : values) {
            map[value] = single_threaded_vector(std::vector(values.begin(), values.end())).GetView();
        }

        return single_threaded_map(std::move(map)).GetView();
    }
};

extern "C"
{
    HRESULT __stdcall make_cpp(void **abi) noexcept
    try
    {
        *abi = detach_abi(make<Implementation>());

        return S_OK;
    }
    catch (...)
    {
        return to_hresult();
    }
}
