#include <cstdio>
#include <winrt/base.h>

void test();

int main()
{
    winrt::init_apartment();

    try
    {
        test();
        return 0;
    }
    catch (winrt::hresult_error const& e)
    {
        std::printf("0x%08X %ls\n", static_cast<uint32_t>(e.code()), e.message().c_str());
    }
    catch (std::exception const& e)
    {
        std::printf("%s\n", e.what());
    }

    return 1;
}
